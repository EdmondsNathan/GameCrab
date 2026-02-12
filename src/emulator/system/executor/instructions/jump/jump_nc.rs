use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
    },
};

impl Console {
    pub(super) fn nc(&mut self) -> Option<u64> {
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
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                );
            }),
        );

        self.push_command(7, Update(Console::command_increment_pc));

        self.push_command(
            8,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(Register8::X),
            ),
        );

        self.push_command(
            9,
            Update(|console: &mut Console| {
                if !console.cpu.get_flag(&Flags::C) {
                    console.push_command(
                        1,
                        Read(
                            Source::Register(Register8::X),
                            Destination::Register(Register8::PcHigh),
                        ),
                    );

                    console.push_command(
                        2,
                        Read(
                            Source::Register(Register8::Y),
                            Destination::Register(Register8::PcLow),
                        ),
                    );

                    console.queue_next_instruction(16 - 9);
                } else {
                    console.queue_next_instruction(12 - 9);
                }
            }),
        );

        // Some(16)
        None
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
    fn jump_nc() {
        let mut console = init(vec![
            (0xD2, 0x100),
            (0x00, 0x101),
            (0x02, 0x102),
            (0x41, 0x200),
        ]);
        console.cpu.set_register(0x45, &Register8::C);
        console.cpu.set_flag(false, &Flags::C);

        for n in 0..20 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 0x45);

        let mut console = init(vec![
            (0xD2, 0x100),
            (0x00, 0x101),
            (0x02, 0x102),
            (0x41, 0x200),
        ]);
        console.cpu.set_register(0x45, &Register8::C);
        console.cpu.set_flag(true, &Flags::C);

        for n in 0..20 {
            console.tick();
        }

        assert_ne!(console.cpu.get_register(&Register8::B), 0x45);
    }
}
