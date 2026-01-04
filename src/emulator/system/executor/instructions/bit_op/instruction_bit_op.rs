use crate::emulator::system::{console::Console, executor::instructions::instruction::BitOps};

impl Console {
    pub(crate) fn instruction_bit_op(&mut self, bit_op: BitOps) -> Option<u64> {
        match bit_op {
            BitOps::Rlca => self.rlca(),
            BitOps::Rla => self.rla(),
            BitOps::Rrca => self.rrca(),
            BitOps::Rra => self.rra(),
            BitOps::Rlc(bit_args) => self.rlc(bit_args),
            BitOps::Rrc(bit_args) => self.rrc(bit_args),
            BitOps::Rl(bit_args) => self.rl(bit_args),
            BitOps::Rr(bit_args) => self.rr(bit_args),
            BitOps::Sla(bit_args) => self.sla(bit_args),
            BitOps::Sra(bit_args) => self.sra(bit_args),
            BitOps::Swap(bit_args) => self.swap(bit_args),
            BitOps::Srl(bit_args) => todo!(),
            BitOps::Bit(_, bit_args) => todo!(),
            BitOps::Reset(_, bit_args) => todo!(),
            BitOps::Set(_, bit_args) => todo!(),
        }
    }
}
