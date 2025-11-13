use crate::emulator::{commands::command::Command, console::Console, instruction::ControlOps};

impl Console {
    pub(in crate::emulator::executor) fn instruction_control(
        &mut self,
        control_op: ControlOps,
    ) -> Option<u64> {
        match control_op {
            ControlOps::Nop => Some(4),
            ControlOps::Stop => todo!(),
            ControlOps::Halt => todo!(),
            ControlOps::Di => todo!(),
            ControlOps::Ei => todo!(),
            ControlOps::Daa => todo!(),
            ControlOps::Scf => todo!(),
            ControlOps::Cpl => todo!(),
            ControlOps::Ccf => todo!(),
        }
    }
}
