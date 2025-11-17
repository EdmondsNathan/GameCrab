use crate::emulator::console::components::{cpu::Cpu, ram::Ram};
use crate::emulator::console::executor::{execute_instruction, execution_queue::ExecutionQueue};
use crate::emulator::rom_loaders::rom::Rom;

pub struct Console {
    pub(crate) cpu: Cpu,
    pub(crate) rom: Rom,
    pub(crate) ram: Ram,
    pub(crate) tick_counter: u64,
    pub(crate) execution_queue: ExecutionQueue,
    pub(crate) cb_flag: bool,
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    pub fn new() -> Console {
        Console {
            cpu: Cpu::new(),
            rom: Rom::new(),
            ram: Ram::new(),
            tick_counter: 0,
            execution_queue: ExecutionQueue::new(),
            cb_flag: false,
        }
    }

    pub fn new_with_rom(path: String) -> Console {
        let mut console = Console::new();
        console.rom = Self::load_rom(path);
        console.rom_into_ram();

        console
    }

    pub fn load_rom(path: String) -> Rom {
        match Rom::try_new(&path) {
            Ok(rom) => rom,
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    fn rom_into_ram(&mut self) {
        let mut i: u16 = 0x100;
        for byte in &self.rom.bytes {
            self.ram.set(*byte, i);
            i += 1;
        }
    }

    pub fn tick(&mut self) {
        if self.tick_counter == 0 {
            self.fetch_decode_execute();
        }

        let map_option = self.execution_queue.pop(&self.tick_counter);
        if let Some(queue) = map_option {
            for command in queue {
                command.execute_command(self);
            }
        }

        self.tick_counter += 1;
    }
}
