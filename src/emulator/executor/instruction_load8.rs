use crate::emulator::{
    console::Console, cpu::CPU, execution_queue::Command, instruction::Ld8, registers::Register8,
};

impl Console {
    pub(super) fn instruction_load8(&mut self, to: Ld8, from: Ld8) -> Option<u64> {
        let to = match to {
            Ld8::A => To::Register(Register8::A),
            Ld8::B => To::Register(Register8::B),
            Ld8::C => To::Register(Register8::C),
            Ld8::D => To::Register(Register8::D),
            Ld8::E => To::Register(Register8::E),
            Ld8::H => To::Register(Register8::H),
            Ld8::L => To::Register(Register8::L),
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

        match from {
            Ld8::A => self.register_from(to, Register8::A),
            Ld8::B => self.register_from(to, Register8::B),
            Ld8::C => self.register_from(to, Register8::C),
            Ld8::D => self.register_from(to, Register8::D),
            Ld8::E => self.register_from(to, Register8::E),
            Ld8::H => self.register_from(to, Register8::H),
            Ld8::L => self.register_from(to, Register8::L),
            Ld8::HL => todo!(),
            Ld8::HLPlus => todo!(),
            Ld8::HLMinus => todo!(),
            Ld8::BC => todo!(),
            Ld8::DE => todo!(),
            Ld8::U16 => todo!(),
            Ld8::U8 => todo!(),
            Ld8::FF00AddU8 => todo!(),
            Ld8::FF00AddC => todo!(),
        }
    }

    fn register_from(&mut self, to: To, from: Register8) -> Option<u64> {
        fn to_register(console: &mut Console, to: Register8, from: Register8) -> Option<u64> {
            console.push_command(
                3,
                Command::SetRegister(CPU::set_register, console.cpu.get_register(from), to),
            );
            Some(4)
        }

        match to {
            To::Register(register8) => to_register(self, register8, from),
        }
    }
}

enum To {
    Register(Register8),
}
