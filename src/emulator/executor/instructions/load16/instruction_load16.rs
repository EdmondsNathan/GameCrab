use crate::emulator::{
    commands::command::{Command::*, Destination, Source},
    console::Console,
    instruction::Ld16,
    registers::{Register16, Register8},
};

impl Console {
    pub(in crate::emulator::executor) fn instruction_load16(&mut self, ld16: Ld16) -> Option<u64> {
        match ld16 {
            Ld16::BcU16 => self.u16_to_register(Register16::Bc),
            Ld16::DeU16 => self.u16_to_register(Register16::De),
            Ld16::HlU16 => self.u16_to_register(Register16::Hl),
            Ld16::SpU16 => self.u16_to_register(Register16::Sp),
            Ld16::U16Sp => self.u16sp(),
            Ld16::SpHl => self.sphl(),
        }
    }

    fn u16_to_register(&mut self, register: Register16) -> Option<u64> {
        let (low, high) = register16_to_register8(register);

        self.push_command(
            3,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(4, Update(Self::command_increment_pc));

        self.push_command(
            5,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(high),
            ),
        );

        self.push_command(
            6,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(7, Update(Self::command_increment_pc));

        self.push_command(
            8,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(low),
            ),
        );

        Some(12)
    }

    fn u16sp(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(4, Update(Self::command_increment_pc));

        self.push_command(
            5,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(Register8::Y),
            ),
        );

        self.push_command(
            6,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(7, Update(Self::command_increment_pc));

        self.push_command(
            8,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(Register8::X),
            ),
        );

        self.push_command(
            9,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Xy),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(
            12,
            Read(
                Source::Register(Register8::SpLow),
                Destination::RamFromRegister(Register16::Bus),
            ),
        );

        self.push_command(
            13,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Xy) + 1,
                    &Register16::Xy,
                );
            }),
        );

        self.push_command(
            14,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Xy),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(
            16,
            Read(
                Source::Register(Register8::SpHigh),
                Destination::RamFromRegister(Register16::Bus),
            ),
        );

        Some(20)
    }

    fn sphl(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Read(
                Source::Register(Register8::H),
                Destination::Register(Register8::SpHigh),
            ),
        );

        self.push_command(
            4,
            Read(
                Source::Register(Register8::L),
                Destination::Register(Register8::SpLow),
            ),
        );

        Some(8)
    }
}

fn register16_to_register8(register: Register16) -> (Register8, Register8) {
    match register {
        Register16::Af => (Register8::A, Register8::F),
        Register16::Bc => (Register8::B, Register8::C),
        Register16::De => (Register8::D, Register8::E),
        Register16::Hl => (Register8::H, Register8::L),
        Register16::Sp => (Register8::SpLow, Register8::SpHigh),
        Register16::Pc => (Register8::PcLow, Register8::PcHigh),
        Register16::Bus => panic!("Bus cannot be split!"),
        Register16::Xy => (Register8::X, Register8::Y),
    }
}
