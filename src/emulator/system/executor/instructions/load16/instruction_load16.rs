use crate::emulator::commands::command::{Command::*, Destination, Source};
use crate::emulator::system::{
    components::registers::*, console::Console, executor::instructions::instruction::*,
};

impl Console {
    pub(crate) fn instruction_load16(&mut self, ld16: Ld16) -> Option<u64> {
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
        let (low, high) = register.register16_to_register8();

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
