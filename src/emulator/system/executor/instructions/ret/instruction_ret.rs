use crate::emulator::{
    commands::command::Command::Update,
    system::{
        components::registers::{Flags, Register16},
        console::Console,
        executor::instructions::instruction::Ret,
    },
};

impl Console {
    pub(crate) fn instruction_ret(&mut self, ret: Ret) -> Option<u64> {
        match ret {
            Ret::None => self.ret(false),
            Ret::I => self.ret(true),
            _ => self.ret_flag(),
        }
    }
}
