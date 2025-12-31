use crate::emulator::system::{console::Console, executor::instructions::instruction::BitOps};

impl Console {
    pub(crate) fn instruction_bit_op(&mut self, bit_op: BitOps) -> Option<u64> {
        todo!()
    }
}
