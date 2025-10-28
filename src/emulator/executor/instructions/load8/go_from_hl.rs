use crate::emulator::{
    console::Console,
    executor::instructions::load8::instruction_load8::{Ff00, Hl, To},
    registers::{Register16, Register8},
};

impl Console {
    pub(super) fn go_from_hl(&mut self, to: To, from: Hl) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: Hl) -> Option<u64> {
            todo!();
        }

        fn to_register16(console: &mut Console, to: Register16, from: Hl) -> Option<u64> {
            todo!();
        }

        fn to_hl(console: &mut Console, to: Hl, from: Hl) -> Option<u64> {
            todo!();
        }

        fn to_u8(console: &mut Console, from: Hl) -> Option<u64> {
            todo!();
        }

        fn to_u16(console: &mut Console, from: Hl) -> Option<u64> {
            todo!();
        }

        fn to_ff00(console: &mut Console, to: Ff00, from: Hl) -> Option<u64> {
            todo!();
        }

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            To::Register16(register16) => to_register16(self, register16, from),
            To::Hl(hl) => to_hl(self, hl, from),
            To::U8 => to_u8(self, from),
            To::U16 => to_u16(self, from),
            To::Ff00(ff00) => to_ff00(self, ff00, from),
        }
    }
}
