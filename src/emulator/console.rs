use crate::emulator::cpu::CPU;
use crate::emulator::execution_request::ExecutionRequest;
use std::collections::VecDeque;

pub struct Console {
    cpu: CPU,
    tick_counter: u64,
    // cpu_execution_queue: VecDeque<ExecutionRequest>,
}

impl Console {
    pub fn new(cpu: CPU) -> Console {
        Console {
            cpu,
            tick_counter: 0,
            // cpu_execution_queue: VecDeque::new(),
        }
    }

    pub fn tick(&mut self) {
        self.tick_counter = self.tick_counter + 1;

        /*match self.cpu_execution_queue.pop_front() {
            None => {
                let byte = self.cpu.ram.fetch(self.cpu.program_counter);
                let instruction = decode(byte);
                match instruction {
                    Ok(instruction) => {
                        self.cpu.execute(instruction);
                    }
                    Err(error) => {
                        panic!("{error}");
                    }
                }
            }
            Some(func) => match func {
                ExecutionRequest::CPUTick(func) => {
                    func(&mut self.cpu);
                }
            },
        }*/
    }
}
