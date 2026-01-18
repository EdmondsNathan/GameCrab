use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
        executor::instructions::instruction::A8Args,
    },
};

impl Console {
    pub(super) fn inc8(&mut self, arg: A8Args) -> Option<u64> {
        match arg {
            A8Args::HL => {
                self.push_command(
                    3,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Hl),
                            &Register16::Bus,
                        );
                    }),
                );

                self.push_command(
                    4,
                    Read(
                        Source::RamFromRegister(Register16::Bus),
                        Destination::Register(Register8::Y),
                    ),
                );

                self.push_command(
                    5,
                    Update(|console: &mut Console| {
                        let original_value = console.cpu.get_register(&Register8::Y);
                        let result = original_value.wrapping_add(1);

                        console.cpu.set_register(result, &Register8::Y);
                    }),
                );

                self.push_command(
                    8,
                    Read(
                        Source::Register(Register8::Y),
                        Destination::RamFromRegister(Register16::Bus),
                    ),
                );

                self.push_command(
                    8,
                    Update(|console: &mut Console| {
                        let result = console.cpu.get_register(&Register8::Y);

                        console.cpu.set_flag(result == 0, &Flags::Z);
                        console.cpu.set_flag(false, &Flags::N);
                        console.cpu.set_flag(result & 0x0F == 0, &Flags::H);
                    }),
                );

                Some(12)
            }
            A8Args::U8 => panic!("Invalid instruction!"),
            // Register8 as the argument
            _ => {
                self.push_command(
                    3,
                    Update(|console: &mut Console| {
                        let register = lookup_register(console);
                        let original_value = console.cpu.get_register(&register);
                        let result = original_value.wrapping_add(1);

                        console.cpu.set_register(result, &register);

                        console.cpu.set_flag(result == 0, &Flags::Z);
                        console.cpu.set_flag(false, &Flags::N);
                        console
                            .cpu
                            .set_flag(original_value & 0x0F == 0x0F, &Flags::H);
                    }),
                );
                Some(4)
            }
        }
    }
}

fn lookup_register(console: &Console) -> Register8 {
    match console
        .ram
        .fetch(console.cpu.get_register_16(&Register16::Bus))
    {
        0x04 => Register8::B,
        0x0C => Register8::C,
        0x14 => Register8::D,
        0x1C => Register8::E,
        0x24 => Register8::H,
        0x2C => Register8::L,
        0x3C => Register8::A,
        _ => panic!("Invalid state!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
    };

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn inc8_hl() {
        let mut console = init(vec![
            (0x34, 0x100),
            (0x34, 0x101),
            (0b00000111, 0x200),
            (0b11111111, 0x201),
        ]);
        console.cpu.set_register_16(0x200, &Register16::Hl);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x200), 0b00001000);
        assert!(!console.cpu.get_flag(&Flags::Z));
        assert!(!console.cpu.get_flag(&Flags::N));
        assert!(!console.cpu.get_flag(&Flags::H));

        console.cpu.set_register_16(0x201, &Register16::Hl);
        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x201), 0);
        assert!(console.cpu.get_flag(&Flags::Z));
        assert!(!console.cpu.get_flag(&Flags::N));
        assert!(console.cpu.get_flag(&Flags::H));
    }

    #[test]
    fn inc8_register8() {
        let mut console = init(vec![(0x04, 0x100), (0x0C, 0x101)]);
        console.cpu.set_register(0b00000111, &Register8::B);
        console.cpu.set_register(0b11111111, &Register8::C);

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 0b00001000);
        assert!(!console.cpu.get_flag(&Flags::Z));
        assert!(!console.cpu.get_flag(&Flags::N));
        assert!(!console.cpu.get_flag(&Flags::H));

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::C), 0);
        assert!(console.cpu.get_flag(&Flags::Z));
        assert!(!console.cpu.get_flag(&Flags::N));
        assert!(console.cpu.get_flag(&Flags::H));
    }
}
