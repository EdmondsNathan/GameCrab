use crate::emulator::{
    console::Console,
    registers::{Register16, Register8},
};

impl Console {
    pub(super) fn command_ram_to_register(&mut self, address: u16, register: Register8) {
        let value = self.ram.fetch(address);

        self.cpu.set_register(value, register);
    }

    pub(super) fn command_register_to_ram(&mut self, address: u16, register: Register8) {
        let value = self.cpu.get_register(register);

        self.ram.set(value, address);
    }

    pub(super) fn command_increment_pc(&mut self) {
        self.cpu
            .set_register_16(self.cpu.get_register_16(Register16::Pc) + 1, Register16::Pc);
    }
}
