use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Register16, Register8},
        console::Console,
    },
};

impl Console {
    pub(super) fn jr_i8(&mut self) -> Option<u64> {
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
                let pc = console.cpu.get_register_16(&Register16::Pc);
                let x = console.cpu.get_register(&Register8::X) as u16;
                let result = pc.wrapping_add(x);

                console.cpu.set_register_16(result, &Register16::Pc);
            }),
        );

        Some(12)
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{
        components::registers::Register16,
        console::{self, Console},
    };

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn jr_i8() {
        let mut console = init(vec![(0x18, 0x100), (0x05, 0x101)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register_16(&Register16::Pc), 0x107);
    }
}
