use crate::emulator::commands::command::{Command::*, Destination, Source};
use crate::emulator::system::{
    components::registers::{Register16, Register8},
    console::Console,
    executor::instructions::load8::instruction_load8::{Ff00, Hl, To},
};

impl Console {
    pub(super) fn go_from_u16(&mut self, to: To) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8) -> Option<u64> {
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
                    Source::RamFromRegister(Register16::Bus),
                    Destination::Register(to),
                ),
            );

            Some(16)
        }

        match to {
            To::Register8(register8) => to_register8(self, register8),
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
    fn to_register_8() {
        let mut console = init(vec![
            (0xFA, 0x100),
            (0x03, 0x101),
            (0x01, 0x102),
            (0x03, 0x103),
        ]);

        for n in 0..16 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x03);
    }
}
