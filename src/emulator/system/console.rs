use crate::emulator::system::components::display::ppu::Ppu;
use crate::emulator::system::components::ram::Interrupts;
use crate::emulator::system::components::registers::{Register16, Register8};
use crate::emulator::system::components::rom::Rom;
use crate::emulator::system::components::{cpu::Cpu, ram::Ram};
use crate::emulator::system::console;
use crate::emulator::system::executor::{execute_instruction, execution_queue::ExecutionQueue};

pub struct Console {
    pub(crate) cpu: Cpu,
    pub(crate) rom: Rom,
    pub(crate) ram: Ram,
    pub(crate) ppu: Ppu,
    pub(crate) tick_counter: u64,
    pub(crate) execution_queue: ExecutionQueue,
    pub(crate) cb_flag: bool,
    previous_div_result: u16,
    tima_overflow_counter: Option<u8>,
}

impl Default for Console {
    fn default() -> Self {
        let mut console = Self {
            cpu: Default::default(),
            rom: Default::default(),
            ram: Default::default(),
            ppu: Default::default(),
            tick_counter: Default::default(),
            execution_queue: Default::default(),
            cb_flag: Default::default(),
            previous_div_result: 0,
            tima_overflow_counter: None,
        };

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
        Console {
            rom: Self::load_rom(path),
            ..Default::default()
        }
    }

    /// Load a rom or panic if none is found.
    fn load_rom(path: &str) -> Rom {
        match Rom::try_new(path) {
            Ok(rom) => rom,
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    /// load the contents of a Rom into Ram.
    pub(crate) fn rom_into_ram(&mut self) {
        for (i, byte) in (0_u16..).zip(self.rom.bytes.iter()) {
            self.ram.set(*byte, i);
        }
    }

    // TAG_TODO Move CPU into its own tick function
    /// Increment the console by one clock cycle.
    pub fn tick(&mut self) {
        self.tick_timers();

        self.tick_ppu();

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
        let tima = self.ram.fetch(0xFF05);
        let tma = self.ram.fetch(0xFF06);
        let tac = self.ram.fetch(0xFF07);

        if let Some(mut count) = self.tima_overflow_counter {
            count += 1;
            self.tima_overflow_counter = Some(count);

            if count == 3 {
                self.tima_overflow_counter = None;

                if tima == 0 {
                    self.ram.set(tma, 0xFF05);
                    self.ram.set_if(true, Interrupts::Timer);
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
            return Some((0x0040, 0x01));
        // LCD Stat
        } else if triggered & 0x02 != 0 {
            // self.handle_interrupt(0x0048, 0x02);
            return Some((0x0048, 0x02));
        // Timer
        } else if triggered & 0x04 != 0 {
            // self.handle_interrupt(0x0050, 0x04);
            return Some((0x0050, 0x04));
        // Serial
        } else if triggered & 0x08 != 0 {
            // self.handle_interrupt(0x0058, 0x08);
            return Some((0x0058, 0x08));
        // Joypad
        } else if triggered & 0x10 != 0 {
            // self.handle_interrupt(0x0060, 0x10);
            return Some((0x0060, 0x10));
        }

        None
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

    pub fn is_interrupt_pending(&self) -> bool {
        let interrupt_enabled = self.ram.fetch(0xFFFF);
        let interrupt_flag = self.ram.fetch(0xFF0F);

        (interrupt_enabled & interrupt_flag & 0x1F) != 0
    }
}
