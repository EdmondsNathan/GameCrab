use crate::emulator::{console::Console, instruction::Ld8, registers::Register8};

impl Console {
    pub(super) fn instruction_load8(&mut self, to: Ld8, from: Ld8) -> Option<u64> {
        let from = match from {
            Ld8::A => todo!(),
            Ld8::B => todo!(),
            Ld8::C => todo!(),
            Ld8::D => todo!(),
            Ld8::E => todo!(),
            Ld8::H => todo!(),
            Ld8::L => todo!(),
            Ld8::HL => todo!(),
            Ld8::HLPlus => todo!(),
            Ld8::HLMinus => todo!(),
            Ld8::BC => todo!(),
            Ld8::DE => todo!(),
            Ld8::U16 => todo!(),
            Ld8::U8 => todo!(),
            Ld8::FF00AddU8 => todo!(),
            Ld8::FF00AddC => todo!(),
        };

        let to = match to {
            Ld8::A => todo!(),
            Ld8::B => todo!(),
            Ld8::C => todo!(),
            Ld8::D => todo!(),
            Ld8::E => todo!(),
            Ld8::H => todo!(),
            Ld8::L => todo!(),
            Ld8::HL => todo!(),
            Ld8::HLPlus => todo!(),
            Ld8::HLMinus => todo!(),
            Ld8::BC => todo!(),
            Ld8::DE => todo!(),
            Ld8::U16 => todo!(),
            Ld8::U8 => todo!(),
            Ld8::FF00AddU8 => todo!(),
            Ld8::FF00AddC => todo!(),
        };

        todo!();
    }
}

enum From {
    Register8(Register8),
}

enum To {
    Register8(Register8),
}
