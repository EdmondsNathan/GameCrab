use crate::emulator::decoder::{decode, decode_cb};
use crate::emulator::executor::execute;
use crate::emulator::ram::RAM;
use crate::emulator::registers::Registers;
use crate::emulator::rom_loaders::rom::ROM;
use std::collections::VecDeque;

pub struct CPU {
    pub(crate) registers: Registers,
    pub(crate) stack_pointer: u16,
    pub(crate) program_counter: u16,
    pub(crate) rom: ROM,
    pub(crate) ram: RAM,
    pub(crate) cb_mode: bool,
    execution_queue: VecDeque<fn(&mut CPU)>,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            stack_pointer: 0xFFFE,
            program_counter: 0x0100,
            rom: ROM::new(),
            ram: RAM::new(),
            cb_mode: false,
            execution_queue: VecDeque::new(),
        }
    }

    pub fn new_rom(path: &str) -> CPU {
        let rom = ROM::try_new(path);
        match rom {
            Ok(rom) => {
                let mut cpu = CPU {
                    registers: Registers::new(),
                    stack_pointer: 0xFFFE,
                    program_counter: 0x0100,
                    rom,
                    ram: RAM::new(),
                    cb_mode: false,
                    execution_queue: VecDeque::new(),
                };
                cpu.rom_into_ram();
                cpu
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        let rom = ROM::try_new(path);
        match rom {
            Ok(rom) => {
                self.rom = rom;
                self.rom_into_ram();
            }
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

    pub(crate) fn tick(&mut self, tick_counter: u64) {
        if self.execution_queue.is_empty() {
            self.fetch_decode_execute();
        }

        if let Some(command) = self.execution_queue.pop_front() {
            command(self);
        }
    }

    pub(crate) fn push_operation(&mut self, operation: fn(&mut CPU)) {
        self.execution_queue.push_back(operation);
    }

    fn fetch_decode_execute(&mut self) {
        let byte = self.ram.fetch(self.program_counter);
        let instruction = if self.cb_mode {
            decode_cb(byte)
        } else {
            decode(byte)
        };

        match instruction {
            Ok(instruction) => {
                execute(self, instruction);
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }
}
