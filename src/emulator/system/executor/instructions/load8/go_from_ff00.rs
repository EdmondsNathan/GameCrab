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
    pub(super) fn go_from_ff00(&mut self, to: To, from: Ff00) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: Ff00) -> Option<u64> {
            match from {
                Ff00::C => return plus_c(console, to),
                Ff00::U8 => return plus_u8(console, to),
            }

            fn plus_c(console: &mut Console, to: Register8) -> Option<u64> {
                console.push_command(
                    3,
                    Read(
                        Source::Register(Register8::C),
                        Destination::Register(Register8::Y),
                    ),
                );

                console.push_command(
                    4,
                    Update(|console: &mut Console| {
                        console.cpu.set_register(0xFF, &Register8::X);
                    }),
                );

                console.push_command(
                    5,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Xy),
                            &Register16::Bus,
                        );
                    }),
                );

                console.push_command(
                    6,
                    Read(
                        Source::RamFromRegister(Register16::Bus),
                        Destination::Register(to),
                    ),
                );

                Some(8)
            }

            fn plus_u8(console: &mut Console, to: Register8) -> Option<u64> {
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
                    Update(|console: &mut Console| {
                        console.cpu.set_register(0xFF, &Register8::X);
                    }),
                );

                console.push_command(
                    7,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Xy),
                            &Register16::Bus,
                        );
                    }),
                );

                console.push_command(
                    8,
                    Read(
                        Source::RamFromRegister(Register16::Bus),
                        Destination::Register(Register8::A),
                    ),
                );

                Some(12)
            }
        }

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            _ => panic!("Invalid instruction!"),
        }
    }
}
