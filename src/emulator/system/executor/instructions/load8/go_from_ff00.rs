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

#[cfg(test)]
mod tests {
    use crate::emulator::system::{components::registers::Register8, console::Console};

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn plus_c() {
        let mut console = init(vec![(0xF2, 0x100), (0x02, 0xFF03)]);
        console.cpu.set_register(0x03, &Register8::C);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x02);
    }

    #[test]
    fn plus_u8() {
        let mut console = init(vec![(0xF0, 0x100), (0x01, 0x101), (0x02, 0xFF01)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x02);
    }
}
