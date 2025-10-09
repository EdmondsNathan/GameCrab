use crate::emulator::{
    console::Console,
    executor::instructions::load_8::instruction_load8::{Hl, To},
};

impl Console {
    pub(super) fn go_from_hl(&mut self, to: To, from: Hl) -> Option<u64> {
        todo!();
    }
}
