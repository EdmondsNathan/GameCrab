use crate::emulator::console::{
    components::registers::{Register16, Register8},
    console::Console,
};

impl Console {
    pub(crate) fn command_increment_pc(&mut self) {
        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Pc) + 1,
            &Register16::Pc,
        );
    }
}
