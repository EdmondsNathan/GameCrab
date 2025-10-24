use crate::emulator::{
    commands::command::Command,
    console::Console,
    cpu::CPU,
    instruction::Ld16,
    registers::{Register16, Register8},
};

impl Console {
    pub(in crate::emulator::executor) fn instruction_load16(&mut self, ld16: Ld16) -> Option<u64> {
        match ld16 {
            Ld16::BCU16 => todo!(),
            Ld16::DEU16 => todo!(),
            Ld16::HLU16 => todo!(),
            Ld16::SPU16 => todo!(),
            Ld16::U16SP => todo!(),
            Ld16::SPHL => todo!(),
        }
    }
}
