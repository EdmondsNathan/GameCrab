use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Flags, Register16, Register8},
        console::{self, Console},
        executor::instructions::instruction::JR,
    },
};

impl Console {
    pub(super) fn jr_flag(&mut self) -> Option<u64> {
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
                Destination::Register(Register8::X),
            ),
        );

        self.push_command(
            6,
            Update(|console: &mut Console| {
                if console.test_flag() {
                    console.push_command(
                        1,
                        Update(|console: &mut Console| {
                            let pc = console.cpu.get_register_16(&Register16::Pc);
                            let x = console.cpu.get_register(&Register8::X) as i8 as i16 as u16;
                            let result = pc.wrapping_add(x);

                            console.cpu.set_register_16(result, &Register16::Pc);
                        }),
                    );

                    console.queue_next_instruction(6);
                } else {
                    console.queue_next_instruction(2);
                }
            }),
        );

        None
    }

    fn test_flag(&self) -> bool {
        match self
            .ram
            .fetch(self.cpu.get_register_16(&Register16::Bus) - 1)
        {
            0x20 => !self.cpu.get_flag(&Flags::Z),
            0x28 => self.cpu.get_flag(&Flags::Z),
            0x30 => !self.cpu.get_flag(&Flags::C),
            0x38 => self.cpu.get_flag(&Flags::C),
            _ => panic!("Impossible value!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{
        components::registers::{Flags, Register8},
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
    fn jr_flag() {
        let mut console = init(vec![
            (0x20, 0x100),
            (0x05, 0x101),
            (0x38, 0x107),
            (0x05, 0x108),
            (0x04, 0x109),
        ]);
        console.cpu.set_flag(false, &Flags::Z);
        console.cpu.set_flag(false, &Flags::C);
        console.cpu.set_register(0x01, &Register8::B);

        for n in 0..24 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 0x02);
    }
}
