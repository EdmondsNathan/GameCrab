use crate::emulator::rom_loaders::rom::Rom;
use crate::emulator::system::components::{cpu::Cpu, ram::Ram};
use crate::emulator::system::executor::{execute_instruction, execution_queue::ExecutionQueue};

#[derive(Default)]
pub struct Console {
    pub(crate) cpu: Cpu,
    pub(crate) rom: Rom,
    pub(crate) ram: Ram,
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
    pub fn new_with_rom(path: String) -> Console {
        Console {
            rom: Self::load_rom(path),
            ..Default::default()
        }
    }

    /// Load a rom or panic if none is found.
    fn load_rom(path: String) -> Rom {
        match Rom::try_new(&path) {
            Ok(rom) => rom,
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    /// load the contents of a Rom into Ram.
    fn rom_into_ram(&mut self) {
        let mut i: u16 = 0x100;
        for byte in &self.rom.bytes {
            self.ram.set(*byte, i);
            i += 1;
        }
    }

    // TAG_REFACTOR Make the tick function call tick on each component
    /// Increment the console by one clock cycle.
    pub fn tick(&mut self) {
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
                //TAG_TODO Perform jumps
                todo!()
            } else {
                // Do not jump for the interrupt, continue on normally
                self.queue_next_instruction(1);
            }
        }

        // Queue the first command.
        if self.tick_counter == 0 {
            self.fetch_decode_execute();
        }

        // execute all commands at the current tick if any exist.
        let map = self.execution_queue.pop(&self.tick_counter);
        if let Some(queue) = map {
            for command in queue {
                command.execute_command(self);
            }
        }

        self.tick_counter += 1;
    }

    pub fn is_interrupt_pending(&self) -> bool {
        let interrupt_enabled = self.ram.fetch(0xFFFF);
        let interrupt_flag = self.ram.fetch(0xFF0F);

        (interrupt_enabled & interrupt_flag) != 0
    }
}
