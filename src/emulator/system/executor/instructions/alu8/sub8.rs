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
    pub(super) fn sub8(&mut self, arg: A8Args) -> Option<u64> {
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
                        let (result, carry) = a_register.overflowing_sub(y_register);
                        let half_carry = (a_register & 0x0F) < (y_register & 0xF);

                        console.cpu.set_register(result, &Register8::A);

                        console.cpu.set_flag(result == 0, &Flags::Z);
                        console.cpu.set_flag(true, &Flags::N);
                        console.cpu.set_flag(half_carry, &Flags::H);
                        console.cpu.set_flag(carry, &Flags::C);
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
                        let (result, carry) = a_register.overflowing_sub(y_register);
                        let half_carry = (a_register & 0x0F) < (y_register & 0xF);

                        console.cpu.set_register(result, &Register8::A);

                        console.cpu.set_flag(result == 0, &Flags::Z);
                        console.cpu.set_flag(true, &Flags::N);
                        console.cpu.set_flag(half_carry, &Flags::H);
                        console.cpu.set_flag(carry, &Flags::C);
                    }),
                );

                Some(8)
            }
            _ => {
                self.push_command(
                    3,
                    Update(|console: &mut Console| {
                        let register = lookup_register(console);
                        let sub_register = console.cpu.get_register(&register);
                        let a_register = console.cpu.get_register(&Register8::A);
                        let (result, carry) = a_register.overflowing_sub(sub_register);
                        let half_carry = (a_register & 0x0F) < (sub_register & 0xF);

                        console.cpu.set_register(result, &Register8::A);

                        console.cpu.set_flag(result == 0, &Flags::Z);
                        console.cpu.set_flag(true, &Flags::N);
                        console.cpu.set_flag(half_carry, &Flags::H);
                        console.cpu.set_flag(carry, &Flags::C);
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
        0x90 => Register8::B,
        0x91 => Register8::C,
        0x92 => Register8::D,
        0x93 => Register8::E,
        0x94 => Register8::H,
        0x95 => Register8::L,
        0x97 => Register8::A,
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
    fn sub8_hl() {
        let mut console = init(vec![(0x96, 0x100), (0x01, 0x200)]);
        console.cpu.set_register(0x10, &Register8::A);
        console.cpu.set_register_16(0x200, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x0F);
        assert!(console.cpu.get_flag(&Flags::H));
    }

    #[test]
    fn sub8_u8() {
        let mut console = init(vec![(0xD6, 0x100), (0x01, 0x101)]);
        console.cpu.set_register(0x10, &Register8::A);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x0F);
        assert!(console.cpu.get_flag(&Flags::H));
    }

    #[test]
    fn sub8_register8() {
        let mut console = init(vec![(0x90, 0x100)]);
        console.cpu.set_register(0x10, &Register8::A);
        console.cpu.set_register(0x01, &Register8::B);

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x0F);
        assert!(console.cpu.get_flag(&Flags::H));
    }
}
