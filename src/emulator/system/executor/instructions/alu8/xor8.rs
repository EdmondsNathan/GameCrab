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
    pub(super) fn xor8(&mut self, arg: A8Args) -> Option<u64> {
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
                        let y_register = console.cpu.get_register(&Register8::Y);
                        let a_register = console.cpu.get_register(&Register8::A);
                        let result = a_register ^ y_register;

                        console.cpu.set_register(result, &Register8::A);

                        console.cpu.set_flag(result == 0, &Flags::Z);
                        console.cpu.set_flag(false, &Flags::N);
                        console.cpu.set_flag(false, &Flags::H);
                        console.cpu.set_flag(false, &Flags::C);
                    }),
                );

                Some(8)
            }
            A8Args::U8 => {
                self.push_command(
                    3,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Pc),
                            &Register16::Bus,
                        );
                    }),
                );

                self.push_command(4, Update(Console::command_increment_pc));

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
                        let y_register = console.cpu.get_register(&Register8::Y);
                        let a_register = console.cpu.get_register(&Register8::A);
                        let result = a_register ^ y_register;

                        console.cpu.set_register(result, &Register8::A);

                        console.cpu.set_flag(result == 0, &Flags::Z);
                        console.cpu.set_flag(false, &Flags::N);
                        console.cpu.set_flag(false, &Flags::H);
                        console.cpu.set_flag(false, &Flags::C);
                    }),
                );

                Some(8)
            }
            _ => {
                self.push_command(
                    3,
                    Update(|console: &mut Console| {
                        let register = lookup_register(console);
                        let and_register = console.cpu.get_register(&register);
                        let a_register = console.cpu.get_register(&Register8::A);
                        let result = a_register ^ and_register;

                        console.cpu.set_register(result, &Register8::A);

                        console.cpu.set_flag(result == 0, &Flags::Z);
                        console.cpu.set_flag(false, &Flags::N);
                        console.cpu.set_flag(false, &Flags::H);
                        console.cpu.set_flag(false, &Flags::C);
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
        0xA8 => Register8::B,
        0xA9 => Register8::C,
        0xAA => Register8::D,
        0xAB => Register8::E,
        0xAC => Register8::H,
        0xAD => Register8::L,
        0xAF => Register8::A,
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
    fn xor8_hl() {
        let mut console = init(vec![(0xAE, 0x100), (0b10001011, 0x200)]);
        console.cpu.set_register(0b11000001, &Register8::A);
        console.cpu.set_register_16(0x200, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0b01001010);
    }

    #[test]
    fn xor8_u8() {
        let mut console = init(vec![(0xEE, 0x100), (0b10001011, 0x101)]);
        console.cpu.set_register(0b11000001, &Register8::A);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0b01001010);
    }

    #[test]
    fn xor8_register8() {
        let mut console = init(vec![(0xA8, 0x100)]);
        console.cpu.set_register(0b11000001, &Register8::A);
        console.cpu.set_register(0b10001011, &Register8::B);

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0b01001010);
        assert!(!console.cpu.get_flag(&Flags::Z));

        let mut console = init(vec![(0xAF, 0x100)]);
        console.cpu.set_register(0b11110001, &Register8::A);

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0b00000000);
        assert!(console.cpu.get_flag(&Flags::Z));
    }
}
