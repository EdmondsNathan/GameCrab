use crate::emulator::commands::command::{Command::*, Destination, Source};
use crate::emulator::system::{
    components::registers::{Register16, Register8},
    console::Console,
    executor::instructions::load8::instruction_load8::{Ff00, Hl, To},
};

impl Console {
    pub(super) fn go_from_register8(&mut self, to: To, from: Register8) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: Register8) -> Option<u64> {
            console.push_command(3, Read(Source::Register(from), Destination::Register(to)));
            Some(4)
        }

        fn to_register16(console: &mut Console, to: Register16, from: Register8) -> Option<u64> {
            // store the value of to into xy so the closure doesn't capture a variable
            console
                .cpu
                .set_register_16(console.cpu.get_register_16(&to), &Register16::Xy);

            console.push_command(
                3,
                Update(|console: &mut Console| {
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Xy),
                        &Register16::Bus,
                    )
                }),
            );

            console.push_command(
                4,
                Read(
                    Source::Register(from),
                    Destination::RamFromRegister(Register16::Bus),
                ),
            );

            Some(8)
        }

        fn to_hl(console: &mut Console, to: Hl, from: Register8) -> Option<u64> {
            console.push_command(
                3,
                Update(|console: &mut Console| {
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Hl),
                        &Register16::Bus,
                    )
                }),
            );

            console.push_command(
                4,
                Read(
                    Source::Register(from),
                    Destination::RamFromRegister(Register16::Bus),
                ),
            );
            match to {
                Hl::Plus => console.push_command(
                    5,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Hl) + 1,
                            &Register16::Hl,
                        );
                    }),
                ),
                Hl::Minus => console.push_command(
                    5,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Hl) - 1,
                            &Register16::Hl,
                        );
                    }),
                ),
            }

            Some(8)
        }

        fn to_u8(console: &mut Console, from: Register8) -> Option<u64> {
            panic!("register 8 to u8 is an invalid instruction!");
        }

        fn to_u16(console: &mut Console, from: Register8) -> Option<u64> {
            console.push_command(
                3,
                Update(|console: &mut Console| {
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Pc),
                        &Register16::Bus,
                    );
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
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Pc),
                        &Register16::Bus,
                    );
                }),
            );

            console.push_command(7, Update(Console::command_increment_pc));

            console.push_command(
                8,
                Read(
                    Source::RamFromRegister(Register16::Bus),
                    Destination::Register(Register8::X),
                ),
            );

            console.push_command(
                9,
                Update(|console: &mut Console| {
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Xy),
                        &Register16::Bus,
                    );
                }),
            );

            console.push_command(
                12,
                Read(
                    Source::Register(Register8::A),
                    Destination::RamFromRegister(Register16::Xy),
                ),
            );

            Some(16)
        }

        fn to_ff00(console: &mut Console, to: Ff00, from: Register8) -> Option<u64> {
            todo!();
        }

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            To::Register16(register16) => to_register16(self, register16, from),
            To::Hl(hl) => to_hl(self, hl, from),
            To::U8 => to_u8(self, from),
            To::U16 => to_u16(self, from),
            To::Ff00(ff00) => to_ff00(self, ff00, from),
        }
    }
}
