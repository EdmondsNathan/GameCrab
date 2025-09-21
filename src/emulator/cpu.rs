use crate::emulator::registers::Registers;

pub(crate) struct CPU {
    pub(crate) registers: Registers,
    pub(crate) stack_pointer: u16,
    pub(crate) program_counter: u16,
    pub(crate) cb_mode: bool,
    pub(crate) enable_interrupts: bool,
}

impl CPU {
    pub(crate) fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            stack_pointer: 0xFFFE,
            program_counter: 0x0100,
            cb_mode: false,
            enable_interrupts: false,
        }
    }
}
