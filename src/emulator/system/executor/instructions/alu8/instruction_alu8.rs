use crate::emulator::system::{console::Console, executor::instructions::instruction::A8Ops};

impl Console {
    pub(crate) fn instruction_alu8(&mut self, op: A8Ops) -> Option<u64> {
        match op {
            A8Ops::Inc(a8_args) => self.inc8(a8_args),
            A8Ops::Dec(a8_args) => self.dec8(a8_args),
            A8Ops::Add(a8_args) => self.add8(a8_args),
            A8Ops::AddCarry(a8_args) => self.adc8(a8_args),
            A8Ops::Sub(a8_args) => self.sub8(a8_args),
            A8Ops::SubCarry(a8_args) => self.sbc8(a8_args),
            A8Ops::And(a8_args) => self.and8(a8_args),
            A8Ops::Or(a8_args) => self.or8(a8_args),
            A8Ops::Xor(a8_args) => self.xor8(a8_args),
            A8Ops::Cmp(a8_args) => todo!(),
        }
    }
}
