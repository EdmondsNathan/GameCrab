use crate::emulator::registers::Registers;

pub(crate) struct CPU {
    pub(crate) registers: Registers,
    pub(crate) cb_mode: bool,
    pub(crate) enable_interrupts: bool,
}

impl CPU {
    pub(crate) fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            cb_mode: false,
            enable_interrupts: false,
        }
    }
}
