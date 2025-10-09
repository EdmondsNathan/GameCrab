use crate::emulator::{console::Console, execution_queue::Command, instruction::ControlOps};

impl Console {
    pub(in crate::emulator::executor) fn instruction_control(
        &mut self,
        control_op: ControlOps,
    ) -> Option<u64> {
        match control_op {
            ControlOps::NOP => Some(4),
            ControlOps::STOP => todo!(),
            ControlOps::HALT => todo!(),
            ControlOps::DI => {
                self.push_command(
                    3,
                    Command::Standard(|console: &mut Console| {
                        console.cpu.enable_interrupts = false;
                    }),
                );
                Some(4)
            }
            ControlOps::EI => {
                self.push_command(
                    3,
                    Command::Standard(|console: &mut Console| {
                        console.cpu.enable_interrupts = false;
                    }),
                );
                Some(4)
            }
            ControlOps::DAA => todo!(),
            ControlOps::SCF => todo!(),
            ControlOps::CPL => todo!(),
            ControlOps::CCF => todo!(),
        }
    }
}
