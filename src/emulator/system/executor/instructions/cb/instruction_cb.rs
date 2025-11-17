use crate::emulator::system::{
    components::registers::Register16, console::Console, executor,
    executor::instructions::cb::instruction_cb,
};

impl Console {
    pub(crate) fn instruction_cb(&mut self) -> Option<u64> {
        self.cb_flag = true;

        Some(4)
    }
}
