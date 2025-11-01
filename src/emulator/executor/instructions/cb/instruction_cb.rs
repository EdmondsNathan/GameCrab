use crate::emulator::{console::Console, decoder::decode_cb, registers::Register16};

impl Console {
    pub(in crate::emulator::executor) fn instruction_cb(&mut self) -> Option<u64> {
        self.cb_flag = true;

        Some(4)
    }
}
