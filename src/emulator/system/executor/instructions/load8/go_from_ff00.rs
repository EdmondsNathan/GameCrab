use crate::emulator::system::{
    components::registers::{Register16, Register8},
    console::Console,
    executor::instructions::load8::instruction_load8::{Ff00, Hl, To},
};

//TAG_TODO
impl Console {
    pub(super) fn go_from_ff00(&mut self, to: To, from: Ff00) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: Ff00) -> Option<u64> {
            match from {
                Ff00::C => return plus_c(),
                Ff00::U8 => return plus_u8(),
            }

            fn plus_c() -> Option<u64> {
                Some(8)
            }

            fn plus_u8() -> Option<u64> {
                Some(12)
            }
        }

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            _ => panic!("Invalid instruction!"),
        }
    }
}
