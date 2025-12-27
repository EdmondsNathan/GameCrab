use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Register16, Register8},
        console::Console,
        executor::instructions::load8::instruction_load8::{Ff00, Hl, To},
    },
};

impl Console {
    pub(super) fn go_from_u8(&mut self, to: To) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8) -> Option<u64> {
            console.push_command(
                3,
                Update(|console: &mut Console| {
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Pc),
                        &Register16::Bus,
                    )
                }),
            );

            console.push_command(4, Update(Console::command_increment_pc));

            console.push_command(
                5,
                Read(
                    Source::RamFromRegister(Register16::Bus),
                    Destination::Register(to),
                ),
            );

            Some(8)
        }

        fn to_register16(console: &mut Console, to: Register16) -> Option<u64> {
            let (low, high) = to.register16_to_register8();

            console.push_command(
                3,
                Update(|console: &mut Console| {
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Pc),
                        &Register16::Bus,
                    )
                }),
            );

            console.push_command(4, Update(Console::command_increment_pc));

            console.push_command(
                5,
                Read(
                    Source::RamFromRegister(Register16::Bus),
                    Destination::Register(Register8::Y),
                ),
            );

            console.push_command(
                6,
                Read(
                    Source::Register(low),
                    Destination::Register(Register8::BusLow),
                ),
            );
            console.push_command(
                6,
                Read(
                    Source::Register(high),
                    Destination::Register(Register8::BusHigh),
                ),
            );

            console.push_command(
                8,
                Read(
                    Source::Register(Register8::Y),
                    Destination::RamFromRegister(Register16::Bus),
                ),
            );

            console.push_command(
                7,
                Read(
                    Source::Register(Register8::Y),
                    Destination::RamFromRegister(Register16::Bus),
                ),
            );

            Some(12)
        }

        match to {
            To::Register8(register8) => to_register8(self, register8),
            To::Register16(register16) => to_register16(self, register16),
            _ => panic!("Invalid instruction!"),
        }
    }
}
