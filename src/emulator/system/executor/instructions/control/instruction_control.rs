use crate::emulator::{
    commands::command::Command, system::console::Console,
    system::executor::instructions::instruction::*,
};

impl Console {
    pub(crate) fn instruction_control(&mut self, control_op: ControlOps) -> Option<u64> {
        match control_op {
            ControlOps::Nop => Some(4),
            ControlOps::Stop => todo!(),
            ControlOps::Halt => todo!(),
            ControlOps::Di => self.di(),
            ControlOps::Ei => self.ei(),
            ControlOps::Daa => todo!(),
            ControlOps::Scf => self.scf(),
            ControlOps::Cpl => self.cpl(),
            ControlOps::Ccf => self.ccf(),
        }
    }
}
