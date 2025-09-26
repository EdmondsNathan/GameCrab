use crate::emulator::{
    console::Console,
    cpu::CPU,
    execution_queue::Command,
    instruction::Ld8,
    registers::{Register16, Register8},
};

enum from_source {
    Register(Register8),
    RamFromRegister16(Register16),
    RamFromU16(u16),
}

enum to_source {
    Register(Register8),
}

impl Console {
    pub(super) fn instruction_load8(&mut self, to: Ld8, from: Ld8) -> Option<u64> {
        let from = match from {
            Ld8::A => from_source::Register(Register8::A),
            Ld8::B => from_source::Register(Register8::B),
            Ld8::C => from_source::Register(Register8::C),
            Ld8::D => from_source::Register(Register8::D),
            Ld8::E => from_source::Register(Register8::E),
            Ld8::H => from_source::Register(Register8::H),
            Ld8::L => from_source::Register(Register8::L),
            Ld8::HL => from_source::RamFromRegister16(Register16::Hl),
            Ld8::HLPlus => from_source::RamFromRegister16(Register16::Hl),
            Ld8::HLMinus => from_source::RamFromRegister16(Register16::Hl),
            Ld8::BC => from_source::RamFromRegister16(Register16::Bc),
            Ld8::DE => from_source::RamFromRegister16(Register16::Bc),
            Ld8::U16 => {
                let address: u16 = self.ram.fetch_16(self.cpu.get_register_16(Register16::Pc));
                from_source::RamFromU16(address)
            }
            Ld8::U8 => todo!(),
            Ld8::FF00AddU8 => todo!(),
            Ld8::FF00AddC => todo!(),
        };

        match from {
            from_source::Register(register8) => todo!(),
            from_source::RamFromRegister16(register16) => todo!(),
            from_source::RamFromU16(_) => todo!(),
        }
    }
}
