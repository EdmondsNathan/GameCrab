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

//TAG_TODO
impl Console {
    pub(super) fn go_from_ff00(&mut self, to: To, from: Ff00) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: Ff00) -> Option<u64> {
            match from {
                Ff00::C => return plus_c(console),
                Ff00::U8 => return plus_u8(),
            }

            fn plus_c(console: &mut Console) -> Option<u64> {
                console.push_command(
                    3,
                    Read(
                        Source::Register(Register8::A),
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
                        Destination::Register(Register8::A),
                    ),
                );

                Some(8)
            }

            fn plus_u8() -> Option<u64> {
                Some(12)
            }
        }

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            _ => panic!("Invalid instruction!"),
        }
    }
}
