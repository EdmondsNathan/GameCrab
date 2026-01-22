use crate::emulator::system::{console::Console, executor::instructions::instruction::A16Ops};

impl Console {
    pub(crate) fn instruction_alu16(&mut self, op: A16Ops) -> Option<u64> {
        match op {
            A16Ops::Inc(a16_args) => self.inc16(),
            A16Ops::Dec(a16_args) => self.dec16(),
            A16Ops::Add(a16_args) => todo!(),
            A16Ops::AddI8 => todo!(),
            A16Ops::LdI8 => todo!(),
        }
    }
}
