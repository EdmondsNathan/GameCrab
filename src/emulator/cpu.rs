use crate::emulator::ram::RAM;
use crate::emulator::registers::Registers;
use crate::emulator::rom_loaders::rom::ROM;

pub struct CPU {
    registers: Registers,
    stack_pointer: u16,
    program_counter: u16,
    rom: ROM,
    ram: RAM,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            stack_pointer: 0xFFFE,
            program_counter: 0x0100,
            rom: ROM::new(),
            ram: RAM::new(),
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
            i = i + 1;
        }
    }
}
