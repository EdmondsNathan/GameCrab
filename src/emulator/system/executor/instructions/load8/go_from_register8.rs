use crate::emulator::commands::command::{Command::*, Destination, Source};
use crate::emulator::system::console;
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
            let (low, high) = to.register16_to_register8();

            console.push_command(
                3,
                Read(
                    Source::Register(low),
                    Destination::Register(Register8::BusHigh),
                ),
            );
            console.push_command(
                3,
                Read(
                    Source::Register(high),
                    Destination::Register(Register8::BusLow),
                ),
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

            // Is it HL Plus or Minus
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
                    Destination::RamFromRegister(Register16::Bus),
                ),
            );

            Some(16)
        }

        fn to_ff00(console: &mut Console, to: Ff00, from: Register8) -> Option<u64> {
            match to {
                Ff00::C => return plus_c(console, from),
                Ff00::U8 => return plus_u8(console, from),
            };

            fn plus_c(console: &mut Console, from: Register8) -> Option<u64> {
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
                        Source::Register(from),
                        Destination::RamFromRegister(Register16::Xy),
                    ),
                );

                Some(8)
            }

            fn plus_u8(console: &mut Console, from: Register8) -> Option<u64> {
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
                        Source::Register(from),
                        Destination::RamFromRegister(Register16::Xy),
                    ),
                );

                Some(12)
            }
        }

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            To::Register16(register16) => to_register16(self, register16, from),
            To::Hl(hl) => to_hl(self, hl, from),
            To::U8 => panic!("Invalid instruction!"),
            To::U16 => to_u16(self, from),
            To::Ff00(ff00) => to_ff00(self, ff00, from),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{
        components::registers::{Register16, Register8},
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
    fn to_register_8() {
        let mut console = init(vec![(0x78, 0x100)]);
        console.cpu.set_register(1, &Register8::B);
        console.cpu.set_register(0, &Register8::A);

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 1);
    }

    #[test]
    fn to_register_16() {
        let mut console = init(vec![(0x02, 0x100)]);
        console.cpu.set_register(0x10, &Register8::A);
        console.cpu.set_register_16(0x256, &Register16::Bc);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x256), 0x10);
    }

    #[test]
    fn to_hl_ldd() {
        let mut console = init(vec![(0x32, 0x100)]);
        console.cpu.set_register(0x10, &Register8::A);
        console.cpu.set_register_16(0x256, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        // Ensure the byte of ram at Hl's location is loaded with register a's value
        assert_eq!(console.ram.fetch(0x256), 0x10);

        // Ensure hl is decremented
        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x255);
    }

    #[test]
    fn to_hl_ldi() {
        let mut console = init(vec![(0x22, 0x100)]);
        console.cpu.set_register(0x10, &Register8::A);
        console.cpu.set_register_16(0x256, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        // Ensure the byte of ram at Hl's location is loaded with register a's value
        assert_eq!(console.ram.fetch(0x256), 0x10);

        // Ensure hl is incremented
        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x257);
    }

    #[test]
    fn to_u16() {
        // EA is the A to u16 instruction
        // the following 2 bytes are 0x01, 0x02
        // Since it is little endian, this corresponds to address 0x0201
        // This address will be set to the value of register A(0x05)
        let mut console = init(vec![(0xEA, 0x100), (0x01, 0x101), (0x02, 0x102)]);
        console.cpu.set_register(0x05, &Register8::A);

        for n in 0..16 {
            console.tick();
        }

        // Ensure the byte of ram at 0x0201 is loaded with register a's value
        assert_eq!(console.ram.fetch(0x0201), 0x05);
    }

    #[test]
    fn to_ff00_c() {
        // Copy the value of A into memory address 0xFF00 + register C

        // 0xE2 LD (0xFF00 + C), A
        let mut console = init(vec![(0xE2, 0x100)]);
        console.cpu.set_register(0x03, &Register8::A);
        console.cpu.set_register(0x05, &Register8::C);

        // 0xE2 is 8 ticks
        for n in 0..8 {
            console.tick();
        }

        // Is Ram 0xFF00 + C == A?
        assert_eq!(console.ram.fetch(0xFF05), 0x03);
    }

    #[test]
    fn to_ff00_u8() {
        // Load the value of A into memory address 0xFF00 + u8

        // 0xE0 LD (0xFF00 + u8), A
        let mut console = init(vec![(0xE0, 0x100), (0x01, 0x101)]);
        console.cpu.set_register(0x03, &Register8::A);

        // 0xE0 is 12 ticks
        for n in 0..12 {
            console.tick();
        }

        // Is Ram 0xFF00 + u8 == A?
        assert_eq!(console.ram.fetch(0xFF01), 0x03);
    }
}
