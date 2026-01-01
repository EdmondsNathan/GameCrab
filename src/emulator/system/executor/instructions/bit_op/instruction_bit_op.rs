use crate::emulator::system::{console::Console, executor::instructions::instruction::BitOps};

impl Console {
    pub(crate) fn instruction_bit_op(&mut self, bit_op: BitOps) -> Option<u64> {
        match bit_op {
            BitOps::Rlca => self.rlca(),
            BitOps::Rla => todo!(),
            BitOps::Rrca => todo!(),
            BitOps::Rra => todo!(),
            BitOps::Rlc(bit_args) => self.rlc(bit_args),
            BitOps::Rrc(bit_args) => self.rrc(bit_args),
            BitOps::Rl(bit_args) => todo!(),
            BitOps::Rr(bit_args) => todo!(),
            BitOps::Sla(bit_args) => todo!(),
            BitOps::Sra(bit_args) => todo!(),
            BitOps::Swap(bit_args) => todo!(),
            BitOps::Srl(bit_args) => todo!(),
            BitOps::Bit(_, bit_args) => todo!(),
            BitOps::Reset(_, bit_args) => todo!(),
            BitOps::Set(_, bit_args) => todo!(),
        }
    }
}
