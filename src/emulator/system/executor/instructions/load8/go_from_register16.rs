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
    pub(super) fn go_from_register16(&mut self, to: To, from: Register16) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: Register16) -> Option<u64> {
            let (high, low) = from.register16_to_register8();

            console.push_command(
                3,
                Read(
                    Source::Register(high),
                    Destination::Register(Register8::BusHigh),
                ),
            );
            console.push_command(
                3,
                Read(
                    Source::Register(low),
                    Destination::Register(Register8::BusLow),
                ),
            );

            console.push_command(
                4,
                Read(
                    Source::RamFromRegister(Register16::Bus),
                    Destination::Register(to),
                ),
            );

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
    fn to_register_8() {
        let mut console = init(vec![(0x0A, 0x100), (0x03, 0x256)]);
        console.cpu.set_register_16(0x256, &Register16::Bc);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x03);
    }
}
