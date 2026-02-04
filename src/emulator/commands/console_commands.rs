use crate::emulator::system::{
    components::registers::{Register16, Register8},
    console::Console,
};

impl Console {
    pub(crate) fn command_increment_pc(&mut self) {
        self.cpu.set_register_16(
            self.cpu.get_register_16(&Register16::Pc).wrapping_add(1),
            &Register16::Pc,
        );
    }
}
