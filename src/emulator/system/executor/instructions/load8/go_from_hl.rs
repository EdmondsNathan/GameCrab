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
    pub(super) fn go_from_hl(&mut self, to: To, from: Hl) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: Hl) -> Option<u64> {
            console.push_command(
                3,
                Update(|console: &mut Console| {
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Hl),
                        &Register16::Bus,
                    );
                }),
            );

            console.push_command(
                4,
                Read(
                    Source::RamFromRegister(Register16::Hl),
                    Destination::Register(to),
                ),
            );

            match from {
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

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            _ => panic!("Invalid instruction!"),
        }
    }
}
