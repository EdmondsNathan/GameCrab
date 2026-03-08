use std::io::{Read, Write};
use std::path::PathBuf;

use crate::emulator::print_logs::log_gameboy_doctor;
use crate::emulator::system::components::audio::apu::Apu;
use crate::emulator::system::components::display::ppu::Ppu;
use crate::emulator::system::components::ram::Interrupts;
use crate::emulator::system::components::registers::{Register16, Register8};
use crate::emulator::system::components::cartridge::Cartridge;
use crate::emulator::system::components::{cpu::Cpu, ram::Ram};
use crate::emulator::system::console;
use crate::emulator::system::executor::{execute_instruction, execution_queue::ExecutionQueue};

pub struct Console {
    pub(crate) cpu: Cpu,
    pub(crate) ram: Ram,
    pub(crate) ppu: Ppu,
    pub(crate) apu: Apu,
    pub(crate) tick_counter: u64,
    pub(crate) execution_queue: ExecutionQueue,
    pub(crate) cb_flag: bool,
    previous_div_result: u16,
    tima_overflow_counter: Option<u8>,
    rom_path: Option<String>,
}

impl Default for Console {
    fn default() -> Self {
        let mut console = Self {
            cpu: Default::default(),
            ram: Default::default(),
            ppu: Default::default(),
            apu: Default::default(),
            tick_counter: Default::default(),
            execution_queue: Default::default(),
            cb_flag: Default::default(),
            previous_div_result: 0,
            tima_overflow_counter: None,
            rom_path: None,
        };

        // Initialize IO registers to post-boot ROM values (DMG)
        console.ram.set(0xCF, 0xFF00); // JOYP
        console.ram.set(0x7E, 0xFF02); // SC
        console.ram.set(0xF8, 0xFF07); // TAC
        console.ram.set(0xE1, 0xFF0F); // IF
        console.ram.set(0x91, 0xFF40); // LCDC
        console.ram.set(0x85, 0xFF41); // STAT
        console.ram.set(0xFC, 0xFF47); // BGP
        console.ram.set(0xFF, 0xFF48); // OBP0
        console.ram.set(0xFF, 0xFF49); // OBP1

        // Initialize APU registers to post-boot values
        console.apu.write_register(0xFF26, 0xF1); // NR52: APU on, ch1+ch2 active
        console.apu.write_register(0xFF11, 0x80); // NR11
        console.apu.write_register(0xFF12, 0xF3); // NR12
        console.apu.write_register(0xFF14, 0xBF); // NR14
        console.apu.write_register(0xFF24, 0x77); // NR50
        console.apu.write_register(0xFF25, 0xF3); // NR51

        console.queue_next_instruction(0);

        console
    }
}

impl Console {
    /// Create a new console object.
    pub fn new() -> Console {
        Self::default()
    }

    /// Create a new console object and load a rom from path.
    pub fn new_with_rom(path: &str) -> Console {
        let bytes = std::fs::read(path).unwrap_or_else(|_| panic!("INVALID ROM PATH: {}", path));
        let cartridge = Cartridge::from_rom(bytes, path);
        let ram = Ram::new(cartridge);

        let mut console = Console {
            ram,
            rom_path: Some(path.to_string()),
            ..Default::default()
        };

        // Re-initialize IO registers (Default sets them, but we replaced ram)
        console.ram.set(0xCF, 0xFF00); // JOYP
        console.ram.set(0x7E, 0xFF02); // SC
        console.ram.set(0xF8, 0xFF07); // TAC
        console.ram.set(0xE1, 0xFF0F); // IF
        console.ram.set(0x91, 0xFF40); // LCDC
        console.ram.set(0x85, 0xFF41); // STAT
        console.ram.set(0xFC, 0xFF47); // BGP
        console.ram.set(0xFF, 0xFF48); // OBP0
        console.ram.set(0xFF, 0xFF49); // OBP1

        // Re-initialize APU registers
        console.apu.write_register(0xFF26, 0xF1); // NR52
        console.apu.write_register(0xFF11, 0x80); // NR11
        console.apu.write_register(0xFF12, 0xF3); // NR12
        console.apu.write_register(0xFF14, 0xBF); // NR14
        console.apu.write_register(0xFF24, 0x77); // NR50
        console.apu.write_register(0xFF25, 0xF3); // NR51

        console
    }

    // TAG_TODO Move CPU into its own tick function
    /// Increment the console by one clock cycle.
    pub fn tick(&mut self) {
        self.tick_timers();

        self.tick_ppu();

        self.tick_apu();

        self.tick_cpu();

        self.tick_counter += 1;
    }

    fn tick_timers(&mut self) {
        if self.cpu.get_is_stopped() {
            // Div resets if cpu is stopped
            self.ram.set(0, 0xFF04);
            return;
        }

        let div = self.ram.get_div().wrapping_add(1);
        let mut tima = self.ram.fetch(0xFF05);
        let tma = self.ram.fetch(0xFF06);
        let tac = self.ram.fetch(0xFF07);

        if let Some(mut count) = self.tima_overflow_counter {
            count += 1;
            self.tima_overflow_counter = Some(count);

            if count == 4 {
                self.tima_overflow_counter = None;

                if tima == 0 {
                    self.ram.set(tma, 0xFF05);
                    self.ram.set_if(true, Interrupts::Timer);
                    tima = tma;
                }
            }
        }

        self.ram.set_div(div);

        let bitshift = match tac & 0b00000011 {
            0b00 => 9_u16,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => panic!("Impossible value!"),
        };

        let timer_enable = ((tac as u16) >> 2) & 1;
        let div_bit = (div >> bitshift) & 1;
        let and_result = timer_enable & div_bit;

        if self.previous_div_result == 1 && and_result == 0 {
            let (result, overflow) = tima.overflowing_add(1);
            self.ram.set(result, 0xFF05);
            if overflow {
                self.tima_overflow_counter = Some(0);
            }
        }

        self.previous_div_result = and_result;
    }

    fn tick_cpu(&mut self) {
        if self.cpu.get_is_stopped() {
            return;
        }

        // Halt bug can only be triggered at the end of an M cycle,
        // so we can handle it immediately
        if self.cpu.get_halt_bug() {
            self.fetch_decode_execute();
            self.cpu.set_halt_bug(false);
            self.cpu.set_halt(false);
        }

        // halt is only checked at the last T cycle of each M cycle
        if self.cpu.get_halt() && self.tick_counter % 4 == 3 && self.is_interrupt_pending() {
            if self.cpu.get_ime() {
                self.end_halt();
            } else {
                // Do not jump for the interrupt, continue on normally
                self.cpu.set_halt(false);
                self.queue_next_instruction(1);
            }
        }

        // execute all commands at the current tick if any exist.
        let map = self.execution_queue.pop(&self.tick_counter);
        if let Some(queue) = map {
            for command in queue {
                command.execute_command(self);
            }
        }
    }

    pub(crate) fn check_interrupts(&mut self) -> Option<(u16, u8)> {
        if !self.cpu.get_ime() {
            return None;
        }

        let if_flag = self.ram.fetch(0xFF0F);
        let ie_flag = self.ram.fetch(0xFFFF);

        // Interrupts are only handled when both IE and IF
        // are set for the specific interrupt
        let triggered = if_flag & ie_flag & 0x1F;

        if triggered == 0 {
            return None;
        }

        // Interrupts are handled according to priority
        // VBlank
        if triggered & 0x01 != 0 {
            // self.handle_interrupt(0x0040, 0x01);
            Some((0x0040, 0x01))
        // LCD Stat
        } else if triggered & 0x02 != 0 {
            // self.handle_interrupt(0x0048, 0x02);
            Some((0x0048, 0x02))
        // Timer
        } else if triggered & 0x04 != 0 {
            // self.handle_interrupt(0x0050, 0x04);
            Some((0x0050, 0x04))
        // Serial
        } else if triggered & 0x08 != 0 {
            // self.handle_interrupt(0x0058, 0x08);
            Some((0x0058, 0x08))
        // Joypad
        } else if triggered & 0x10 != 0 {
            // self.handle_interrupt(0x0060, 0x10);
            Some((0x0060, 0x10))
        } else {
            None
        }
    }

    pub(crate) fn handle_interrupt(&mut self, address: u16, bit: u8) {
        self.cpu.set_ime(false);

        // Clear the IF bit for this interrupt
        self.ram.set(self.ram.fetch(0xFF0F) & !bit, 0xFF0F);

        // Push the current PC onto the stack (SP--, [SP] = high, SP--, [SP] = low)
        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
            &Register16::Sp,
        );

        self.ram.set(
            self.cpu.get_register(&Register8::PcHigh),
            self.cpu.get_register_16(&Register16::Sp),
        );

        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
            &Register16::Sp,
        );

        self.ram.set(
            self.cpu.get_register(&Register8::PcLow),
            self.cpu.get_register_16(&Register16::Sp),
        );

        // Jump PC to handle the interrupt
        self.cpu.set_register_16(address, &Register16::Pc);

        self.queue_next_instruction(20);
    }

    fn end_halt(&mut self) {
        // TAG_TODO Perform jumps
        // TAG_TODO Convert to queued commands
        // https://gbdev.io/pandocs/Interrupts.html
        self.cpu.set_halt(false);

        let interrupt_mask = self.ram.fetch(0xFFFF) & self.ram.fetch(0xFF0F);
        let interrupt_index = interrupt_mask.trailing_zeros() as u16;
        let interrupt_flag = self.ram.fetch(0xFF0F);
        self.ram
            .set(interrupt_flag & !(1 << interrupt_index), 0xFF0F);

        self.cpu.set_ime(false);

        // Push PC onto stack (SP--, [SP] = high, SP--, [SP] = low)
        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
            &Register16::Sp,
        );

        self.ram.set(
            self.cpu.get_register(&Register8::PcHigh),
            self.cpu.get_register_16(&Register16::Sp),
        );

        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
            &Register16::Sp,
        );

        self.ram.set(
            self.cpu.get_register(&Register8::PcLow),
            self.cpu.get_register_16(&Register16::Sp),
        );

        self.cpu
            .set_register_16(0x0040 + interrupt_index * 8, &Register16::Pc);
        self.queue_next_instruction(21);
    }

    /// Update joypad state and request joypad interrupt if a button was newly pressed.
    pub fn update_joypad(&mut self, action: u8, direction: u8) {
        let old_action = self.ram.joypad_action;
        let old_direction = self.ram.joypad_direction;
        self.ram.set_joypad(action, direction);

        // A button went from released (1) to pressed (0) — fire joypad interrupt
        let newly_pressed = (old_action & !action) | (old_direction & !direction);
        if newly_pressed != 0 {
            self.ram.set_if(true, Interrupts::Joypad);
        }
    }

    pub fn save_state(&self) {
        let path = match &self.rom_path {
            Some(p) => {
                let mut pb = PathBuf::from(p);
                pb.set_extension("state");
                pb
            }
            None => {
                eprintln!("Cannot save state: no ROM loaded");
                return;
            }
        };

        let result = (|| -> std::io::Result<()> {
            let mut file = std::fs::File::create(&path)?;
            file.write_all(b"GCSS")?;
            file.write_all(&[0x02])?;
            self.cpu.save_state(&mut file)?;
            self.ram.save_state(&mut file)?;
            self.ppu.save_state(&mut file)?;
            self.apu.save_state(&mut file)?;
            file.write_all(&self.tick_counter.to_le_bytes())?;
            file.write_all(&[self.cb_flag as u8])?;
            file.write_all(&self.previous_div_result.to_le_bytes())?;
            let tima_bytes = match self.tima_overflow_counter {
                Some(v) => [1, v],
                None => [0, 0],
            };
            file.write_all(&tima_bytes)?;
            Ok(())
        })();

        match result {
            Ok(()) => eprintln!("State saved to {:?}", path),
            Err(e) => eprintln!("Failed to save state: {}", e),
        }
    }

    pub fn load_state(&mut self) {
        let path = match &self.rom_path {
            Some(p) => {
                let mut pb = PathBuf::from(p);
                pb.set_extension("state");
                pb
            }
            None => {
                eprintln!("Cannot load state: no ROM loaded");
                return;
            }
        };

        let result = (|| -> std::io::Result<()> {
            let mut file = std::fs::File::open(&path)?;
            let mut magic = [0u8; 4];
            file.read_exact(&mut magic)?;
            if &magic != b"GCSS" {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid save state magic",
                ));
            }
            let mut version = [0u8; 1];
            file.read_exact(&mut version)?;
            if version[0] != 0x01 && version[0] != 0x02 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Unsupported save state version: {}", version[0]),
                ));
            }
            self.cpu.load_state(&mut file)?;
            self.ram.load_state(&mut file)?;
            self.ppu.load_state(&mut file)?;
            if version[0] >= 0x02 {
                self.apu.load_state(&mut file)?;
            }

            let mut buf8 = [0u8; 8];
            file.read_exact(&mut buf8)?;
            self.tick_counter = u64::from_le_bytes(buf8);

            let mut cb = [0u8; 1];
            file.read_exact(&mut cb)?;
            self.cb_flag = cb[0] != 0;

            let mut div_buf = [0u8; 2];
            file.read_exact(&mut div_buf)?;
            self.previous_div_result = u16::from_le_bytes(div_buf);

            let mut tima = [0u8; 2];
            file.read_exact(&mut tima)?;
            self.tima_overflow_counter = if tima[0] != 0 { Some(tima[1]) } else { None };

            self.execution_queue.clear();
            self.queue_next_instruction(0);

            Ok(())
        })();

        match result {
            Ok(()) => eprintln!("State loaded from {:?}", path),
            Err(e) => eprintln!("Failed to load state: {}", e),
        }
    }

    pub fn save_ram(&mut self) {
        self.ram.save_ram();
    }

    pub fn is_interrupt_pending(&self) -> bool {
        let interrupt_enabled = self.ram.fetch(0xFFFF);
        let interrupt_flag = self.ram.fetch(0xFF0F);

        (interrupt_enabled & interrupt_flag & 0x1F) != 0
    }
}
