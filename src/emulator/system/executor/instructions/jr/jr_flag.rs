use crate::emulator::system::{console::Console, executor::instructions::instruction::JR};

impl Console {
    pub(super) fn jr_flag(&mut self, jr: JR) -> Option<u64> {
        todo!()
    }
}
