use crate::emulator::system::{console::Console, executor::instructions::instruction::JP};

impl Console {
    pub(crate) fn instruction_jump(&mut self, jump: JP) -> Option<u64> {
        match jump {
            JP::U16 => self.u16(),
            JP::HL => self.hl(),
            JP::Nz => self.nz(),
            JP::Nc => self.nc(),
            JP::Z => self.z(),
            JP::C => self.c(),
        }
    }
}
