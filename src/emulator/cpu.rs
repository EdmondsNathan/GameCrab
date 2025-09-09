use crate::emulator::registers::Registers;
use crate::emulator::ram::RAM;

pub struct CPU {
    registers: Registers,
    stack_pointer: u16,
    program_counter: u16,
    ram: RAM,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            stack_pointer: 0xFFFE,
            program_counter: 0x0100,
            ram: RAM::new(),
        }
    }
}