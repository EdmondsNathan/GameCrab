use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Register16, Register8},
        console::Console,
        executor::instructions::instruction::PushPop,
    },
};

impl Console {
    pub(crate) fn stack_pop16(&mut self, arg: PushPop) -> Option<u64> {
        let (high, low) = match arg {
            PushPop::Bc => Register16::Bc,
            PushPop::De => Register16::De,
            PushPop::Hl => Register16::Hl,
            PushPop::Af => Register16::Af,
        }
        .register16_to_register8();

        self.push_command(3, Update(Console::command_increment_pc));

        self.push_command(
            4,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp),
                    &Register16::Bus,
                );
            }),
        );

        self.push_command(
            5,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(low),
            ),
        );

        self.push_command(
            6,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp) + 1,
                    &Register16::Sp,
                );
            }),
        );

        self.push_command(
            7,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp),
                    &Register16::Bus,
                );
            }),
        );

        self.push_command(
            8,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(high),
            ),
        );

        Some(12)
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
    fn stack_pop() {
        let mut console = init(vec![(0xC1, 0x100), (0x45, 0x200), (0x67, 0x201)]);
        console.cpu.set_register_16(0x200, &Register16::Sp);

        for n in 0..16 {
            console.tick();
        }

        assert_eq!(
            console.cpu.get_register(&Register8::B),
            console.ram.fetch(0x201)
        );
        assert_eq!(
            console.cpu.get_register(&Register8::C),
            console.ram.fetch(0x200)
        );
    }
}
