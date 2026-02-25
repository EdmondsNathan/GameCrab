use crate::emulator::system::components::display::ppu::Ppu;
use crate::emulator::system::components::ram::Interrupts;
use crate::emulator::system::components::registers::{Register16, Register8};
use crate::emulator::system::components::rom::Rom;
use crate::emulator::system::components::{cpu::Cpu, ram::Ram};
use crate::emulator::system::console;
use crate::emulator::system::executor::{execute_instruction, execution_queue::ExecutionQueue};

#[derive(Default)]
pub struct Console {
    pub(crate) cpu: Cpu,
    pub(crate) rom: Rom,
    pub(crate) ram: Ram,
    pub(crate) ppu: Ppu,
    pub(crate) tick_counter: u64,
    pub(crate) execution_queue: ExecutionQueue,
    pub(crate) cb_flag: bool,
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
        // self.tick_ppu();

        self.check_interrupts();

        self.tick_cpu();

        self.tick_counter += 1;
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

        // // Queue the first command.
        // if self.tick_counter == 0 {
        //     self.fetch_decode_execute();
        // }

        // execute all commands at the current tick if any exist.
        let map = self.execution_queue.pop(&self.tick_counter);
        if let Some(queue) = map {
            for command in queue {
                command.execute_command(self);
            }
        }
    }

    fn check_interrupts(&mut self) {
        // if !self.cpu.get_ime() {
        //     return;
        // }
        //
        // let if_flag = self.ram.fetch(0xFF0F);
        // let ie_flag = self.ram.fetch(0xFFFF);
        //
        // let triggered = if_flag & ie_flag & 0x1F;
        //
        // if triggered == 0 {
        //     return;
        // }
        //
        // // Interrupts are handled according to priority
        // // VBlank
        // if triggered & 0x01 != 0 {
        //     self.handle_interrupt(0x0040, 0x01);
        // // LCD Stat
        // } else if triggered & 0x02 != 0 {
        //     self.handle_interrupt(0x0048, 0x02);
        // // Timer
        // } else if triggered & 0x04 != 0 {
        //     self.handle_interrupt(0x0050, 0x04);
        // // Serial
        // } else if triggered & 0x08 != 0 {
        //     self.handle_interrupt(0x0058, 0x08);
        // // Joypad
        // } else if triggered & 0x10 != 0 {
        //     self.handle_interrupt(0x0060, 0x10);
        // }
    }

    fn handle_interrupt(&mut self, address: u16, bit: u8) {
        // Clear
        self.cpu.set_ime(false);
        self.cpu.set_ime_pending(false);

        // Clear the IF bit for this interrupt
        self.ram.set(self.ram.fetch(0xFF0F) & !bit, 0xFF0F);

        // Push the current PC onto the stack
        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
            &Register16::Sp,
        );

        self.ram.set(
            (self.cpu.get_register_16(&Register16::Pc) >> 8) as u8,
            self.cpu.get_register_16(&Register16::Sp),
        );

        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
            &Register16::Sp,
        );

        self.ram.set(
            (self.cpu.get_register_16(&Register16::Pc) & 0xFF) as u8,
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

        self.cpu
            .set_register_16(self.cpu.get_register_16(&Register16::Sp), &Register16::Bus);

        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
            &Register16::Sp,
        );

        self.ram.set(
            self.cpu.get_register(&Register8::PcHigh),
            self.cpu.get_register_16(&Register16::Bus),
        );

        self.cpu
            .set_register_16(self.cpu.get_register_16(&Register16::Sp), &Register16::Bus);

        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Sp) - 1,
            &Register16::Sp,
        );

        self.ram.set(
            self.cpu.get_register(&Register8::PcLow),
            self.cpu.get_register_16(&Register16::Bus),
        );

        self.cpu
            .set_register_16(0x0040 + interrupt_index * 8, &Register16::Pc);
        self.queue_next_instruction(19);
    }

    pub fn is_interrupt_pending(&self) -> bool {
        let interrupt_enabled = self.ram.fetch(0xFFFF);
        let interrupt_flag = self.ram.fetch(0xFF0F);

        (interrupt_enabled & interrupt_flag) != 0
    }
}
