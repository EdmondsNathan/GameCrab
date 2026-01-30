use crate::emulator::system::{console::Console, executor::instructions::instruction::JR};

impl Console {
    pub(crate) fn instruction_jr(&mut self, jr: JR) -> Option<u64> {
        if let JR::I8 = jr {
            self.jr_i8()
        } else {
            self.jr_flag(jr)
        }
    }
}
