use crate::emulator::cpu::CPU;

pub struct Console {
    cpu: CPU,
    tick_counter: u64,
}

impl Console {
    pub fn new(cpu: CPU) -> Console {
        Console {
            cpu,
            tick_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;
    }
}
