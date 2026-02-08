use crate::emulator::system::{
    components::registers::{Register16, Register8},
    console::Console,
    executor::instructions::instruction::Ld8,
};

impl Console {
    pub(crate) fn instruction_load8(&mut self, to: Ld8, from: Ld8) -> Option<u64> {
        let to = match to {
            Ld8::A => To::Register8(Register8::A),
            Ld8::B => To::Register8(Register8::B),
            Ld8::C => To::Register8(Register8::C),
            Ld8::D => To::Register8(Register8::D),
            Ld8::E => To::Register8(Register8::E),
            Ld8::H => To::Register8(Register8::H),
            Ld8::L => To::Register8(Register8::L),
            Ld8::Hl => To::Register16(Register16::Hl),
            Ld8::HlPlus => To::Hl(Hl::Plus),
            Ld8::HlMinus => To::Hl(Hl::Minus),
            Ld8::Bc => To::Register16(Register16::Bc),
            Ld8::De => To::Register16(Register16::De),
            Ld8::U16 => To::U16,
            Ld8::U8 => To::U8,
            Ld8::Ff00AddU8 => To::Ff00(Ff00::U8),
            Ld8::Ff00AddC => To::Ff00(Ff00::C),
        };

        match from {
            Ld8::A => self.go_from_register8(to, Register8::A),
            Ld8::B => self.go_from_register8(to, Register8::B),
            Ld8::C => self.go_from_register8(to, Register8::C),
            Ld8::D => self.go_from_register8(to, Register8::D),
            Ld8::E => self.go_from_register8(to, Register8::E),
            Ld8::H => self.go_from_register8(to, Register8::H),
            Ld8::L => self.go_from_register8(to, Register8::L),
            Ld8::Hl => self.go_from_register16(to, Register16::Hl),
            Ld8::HlPlus => self.go_from_hl(to, Hl::Plus),
            Ld8::HlMinus => self.go_from_hl(to, Hl::Minus),
            Ld8::Bc => self.go_from_register16(to, Register16::Bc),
            Ld8::De => self.go_from_register16(to, Register16::De),
            Ld8::U16 => self.go_from_u16(to),
            Ld8::U8 => self.go_from_u8(to),
            Ld8::Ff00AddU8 => self.go_from_ff00(to, Ff00::U8),
            Ld8::Ff00AddC => self.go_from_ff00(to, Ff00::C),
        }
    }
}

pub(super) enum To {
    Register8(Register8),
    Register16(Register16),
    Hl(Hl),
    U8,
    U16,
    Ff00(Ff00),
}

pub(super) enum Hl {
    Plus,
    Minus,
}

pub(super) enum Ff00 {
    C,
    U8,
}
