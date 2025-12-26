use crate::emulator::system::{
    components::registers::{Register16, Register8},
    console::Console,
    executor::instructions::load8::instruction_load8::{Ff00, Hl, To},
};

impl Console {
    pub(super) fn go_from_u16(&mut self, to: To, from: u16) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: u16) -> Option<u64> {
            todo!();
        }

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            To::Register16(register16) => panic!("Invalid instruction!"),
            To::Hl(hl) => panic!("Invalid instruction!"),
            To::U8 => panic!("Invalid instruction!"),
            To::U16 => panic!("Invalid instruction!"),
            To::Ff00(ff00) => panic!("Invalid instruction!"),
        }
    }
}
