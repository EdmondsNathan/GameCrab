use crate::emulator::{
    console::Console,
    cpu::CPU,
    execution_queue::Command,
    instruction::Ld8,
    registers::{Register16, Register8},
};

impl Console {
    pub(in crate::emulator::executor) fn instruction_load8(
        &mut self,
        to: Ld8,
        from: Ld8,
    ) -> Option<u64> {
        let to = match to {
            Ld8::A => To::Register8(Register8::A),
            Ld8::B => To::Register8(Register8::B),
            Ld8::C => To::Register8(Register8::C),
            Ld8::D => To::Register8(Register8::D),
            Ld8::E => To::Register8(Register8::E),
            Ld8::H => To::Register8(Register8::H),
            Ld8::L => To::Register8(Register8::L),
            Ld8::HL => To::Register16(Register16::Hl),
            Ld8::HLPlus => To::Hl(Hl::Plus),
            Ld8::HLMinus => To::Hl(Hl::Minus),
            Ld8::BC => To::Register16(Register16::Bc),
            Ld8::DE => To::Register16(Register16::De),
            Ld8::U16 => To::U16,
            Ld8::U8 => To::U8,
            Ld8::FF00AddU8 => To::Ff00(Ff00::U8),
            Ld8::FF00AddC => To::Ff00(Ff00::C),
        };

        match from {
            Ld8::A => self.go_from_register8(to, Register8::A),
            Ld8::B => self.go_from_register8(to, Register8::B),
            Ld8::C => self.go_from_register8(to, Register8::C),
            Ld8::D => self.go_from_register8(to, Register8::D),
            Ld8::E => self.go_from_register8(to, Register8::E),
            Ld8::H => self.go_from_register8(to, Register8::H),
            Ld8::L => self.go_from_register8(to, Register8::L),
            Ld8::HL => self.go_from_register16(to, Register16::Hl),
            Ld8::HLPlus => todo!(),
            Ld8::HLMinus => todo!(),
            Ld8::BC => self.go_from_register16(to, Register16::Bc),
            /*0A
            self.push_command(
                4,
                Command::SetRegister(
                    CPU::set_register,
                    self.ram.fetch(self.cpu.get_register_16(Register16::Bc)),
                    Register8::A,
                ),
            );
            Some(8)*/
            Ld8::DE => self.go_from_register16(to, Register16::De),
            Ld8::U16 => todo!(),
            Ld8::U8 => todo!(),
            Ld8::FF00AddU8 => todo!(),
            Ld8::FF00AddC => todo!(),
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
