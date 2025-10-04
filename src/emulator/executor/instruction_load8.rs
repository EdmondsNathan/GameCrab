use crate::emulator::{console::Console, instruction::Ld8, registers::Register8};

impl Console {
    pub(super) fn instruction_load8(&mut self, to: Ld8, from: Ld8) -> Option<u64> {
        let from = match from {
            Ld8::A => From::Register(Register8::A),
            Ld8::B => From::Register(Register8::B),
            Ld8::C => From::Register(Register8::C),
            Ld8::D => From::Register(Register8::D),
            Ld8::E => From::Register(Register8::E),
            Ld8::H => From::Register(Register8::H),
            Ld8::L => From::Register(Register8::L),
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
            Ld8::A => To::Register(Register8::A),
            Ld8::B => To::Register(Register8::A),
            Ld8::C => To::Register(Register8::A),
            Ld8::D => To::Register(Register8::A),
            Ld8::E => To::Register(Register8::A),
            Ld8::H => To::Register(Register8::A),
            Ld8::L => To::Register(Register8::A),
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

        todo!()
    }
}

enum From {
    Register(Register8),
    Address,
}

enum To {
    Register(Register8),
}
