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
    fn hl_plus() {
        let mut console = init(vec![(0x2A, 0x100), (0x05, 0x103)]);
        console.cpu.set_register_16(0x103, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x05);
        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x104);
    }

    #[test]
    fn hl_minus() {
        let mut console = init(vec![(0x3A, 0x100), (0x05, 0x103)]);
        console.cpu.set_register_16(0x103, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x05);
        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x102);
    }
}
