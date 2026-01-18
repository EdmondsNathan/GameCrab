use crate::emulator::system::{console::Console, executor::instructions::instruction::A8Ops};

impl Console {
    pub(crate) fn instruction_alu8(&mut self, op: A8Ops) -> Option<u64> {
        match op {
            A8Ops::Inc(a8_args) => self.inc8(a8_args),
            A8Ops::Dec(a8_args) => self.dec8(a8_args),
            A8Ops::Add(a8_args) => todo!(),
            A8Ops::AddCarry(a8_args) => todo!(),
            A8Ops::Sub(a8_args) => todo!(),
            A8Ops::SubCarry(a8_args) => todo!(),
            A8Ops::And(a8_args) => todo!(),
            A8Ops::Or(a8_args) => todo!(),
            A8Ops::Xor(a8_args) => todo!(),
            A8Ops::Cmp(a8_args) => todo!(),
        }
    }
}
