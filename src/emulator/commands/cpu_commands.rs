use crate::emulator::{
    cpu::CPU,
    registers::{Flags, Register8},
};

impl CPU {
    pub(crate) fn cpu_add(&mut self, value: u8, register: &Register8, flags: bool) {
        let original = self.get_register(register);
        let (new, overflow) = original.overflowing_add(value);
        self.set_register(new, register);

        if !flags {
            return;
        }

        self.set_flag(self.get_register(register) == 0, &Flags::Z);
        self.set_flag(false, &Flags::N);
        self.set_flag((original & 0xF) + (value & 0xF) > 0xF, &Flags::H);
        self.set_flag(overflow, &Flags::C);
    }

        todo!();
    pub(crate) fn cpu_adc(&mut self, value: u8, register: &Register8, flags: bool) {
    }

    pub(crate) fn cpu_sub(&mut self, value: u8, register: &Register8, flags: bool) {
        todo!();
    }

    pub(crate) fn cpu_sbc(&mut self, value: u8, register: &Register8, flags: bool) {
        todo!();
    }
}
