use crate::emulator::system::{console::Console, executor::instructions::instruction::JP};

impl Console {
    pub(crate) fn instruction_jump(&mut self, jump: JP) -> Option<u64> {
        match jump {
            JP::U16 => todo!(),
            JP::HL => self.hl(),
            JP::Nz => todo!(),
            JP::Nc => todo!(),
            JP::Z => todo!(),
            JP::C => todo!(),
        }
    }
}
