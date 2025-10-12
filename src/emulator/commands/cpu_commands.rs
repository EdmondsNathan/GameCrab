use crate::emulator::{
    cpu::CPU,
    registers::{Flags, Register8},
};

impl CPU {
    pub(crate) fn cpu_add(&mut self, value: u8, register: &Register8, flags: bool) {
        todo!();
    }

    pub(crate) fn cpu_adc(&mut self, value: u8, register: Register8, flags: bool) {
        todo!();
    }

    pub(crate) fn cpu_sub(&mut self, value: u8, register: Register8, flags: bool) {
        todo!();
    }

    pub(crate) fn cpu_sbc(&mut self, value: u8, register: Register8, flags: bool) {
        todo!();
    }
}
