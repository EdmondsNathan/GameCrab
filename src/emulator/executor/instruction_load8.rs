use crate::emulator::{
    console::Console,
    cpu::CPU,
    execution_queue::Command,
    instruction::Ld8,
    registers::{Register16, Register8},
};

impl Console {
    pub(super) fn instruction_load8(&mut self, to: Ld8, from: Ld8) -> Option<u64> {
        let to = match to {
            Ld8::A => To::Register8(Register8::A),
            Ld8::B => To::Register8(Register8::B),
            Ld8::C => To::Register8(Register8::C),
            Ld8::D => To::Register8(Register8::D),
            Ld8::E => To::Register8(Register8::E),
            Ld8::H => To::Register8(Register8::H),
            Ld8::L => To::Register8(Register8::L),
            Ld8::HL => todo!(),
            Ld8::HLPlus => todo!(),
            Ld8::HLMinus => todo!(),
            Ld8::BC => To::Register16(Register16::Bc),
            Ld8::DE => To::Register16(Register16::De),
            Ld8::U16 => todo!(),
            Ld8::U8 => todo!(),
            Ld8::FF00AddU8 => todo!(),
            Ld8::FF00AddC => todo!(),
        };

        match from {
            Ld8::A => self.go_from_register8(to, Register8::A),
            Ld8::B => self.go_from_register8(to, Register8::B),
            Ld8::C => self.go_from_register8(to, Register8::C),
            Ld8::D => self.go_from_register8(to, Register8::D),
            Ld8::E => self.go_from_register8(to, Register8::E),
            Ld8::H => self.go_from_register8(to, Register8::H),
            Ld8::L => self.go_from_register8(to, Register8::L),
            Ld8::HL => todo!(),
            Ld8::HLPlus => todo!(),
            Ld8::HLMinus => todo!(),
            Ld8::BC => {
                //0A
                self.push_command(
                    4,
                    Command::SetRegister(
                        CPU::set_register,
                        self.ram.fetch(self.cpu.get_register_16(Register16::Bc)),
                        Register8::A,
                    ),
                );
                Some(8)
            }
            Ld8::DE => todo!(),
            Ld8::U16 => todo!(),
            Ld8::U8 => todo!(),
            Ld8::FF00AddU8 => todo!(),
            Ld8::FF00AddC => todo!(),
        }
    }

    fn go_from_register8(&mut self, to: To, from: Register8) -> Option<u64> {
        fn to_register(console: &mut Console, to: Register8, from: Register8) -> Option<u64> {
            console.push_command(
                3,
                Command::SetRegister(CPU::set_register, console.cpu.get_register(from), to),
            );
            Some(4)
        }

        fn to_register16(console: &mut Console, to: Register16, from: Register8) -> Option<u64> {
            console.push_command(
                4,
                Command::SetRegister(
                    CPU::set_register,
                    console.ram.fetch(console.cpu.get_register_16(to)),
                    from,
                ),
            );
            Some(8)
        }

        match to {
            To::Register8(register8) => to_register(self, register8, from),
            To::Register16(register16) => to_register16(self, register16, from),
        }
    }
}

enum To {
    Register8(Register8),
    Register16(Register16),
}
