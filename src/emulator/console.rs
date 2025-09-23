use crate::emulator::{cpu::CPU, execution_queue::ExecutionQueue, ram::RAM, rom_loaders::rom::ROM};

pub struct Console {
    pub(crate) cpu: CPU,
    pub(crate) rom: ROM,
    pub(crate) ram: RAM,
    pub(crate) tick_counter: u64,
    pub(crate) execution_queue: ExecutionQueue,
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    pub fn new() -> Console {
        let mut new_console = Console {
            cpu: CPU::new(),
            rom: ROM::new(),
            ram: RAM::new(),
            tick_counter: 0,
            execution_queue: ExecutionQueue::new(),
        };
        new_console
            .execution_queue
            .push_command(0, |console: &mut Console| {
                console.command_fetch_decode_execute()
            });
        new_console
    }

    pub fn new_with_rom(path: String) -> Console {
        let mut console = Console::new();
        console.rom = Self::load_rom(path);
        console.rom_into_ram();

        console
    }

    pub fn load_rom(path: String) -> ROM {
        match ROM::try_new(&path) {
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
        self.run_commands();
        self.tick_counter += 1;
    }

    fn run_commands(&mut self) {
        let map_option = self.execution_queue.pop(&self.tick_counter);
        if let Some(queue) = map_option {
            for command in queue {
                command(self);
            }
        }
    }
}
