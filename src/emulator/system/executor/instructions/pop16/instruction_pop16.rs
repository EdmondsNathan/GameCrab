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
            PushPop::Bc => (Register8::B, Register8::C),
            PushPop::De => (Register8::D, Register8::E),
            PushPop::Hl => (Register8::H, Register8::L),
            PushPop::Af => (Register8::A, Register8::F),
        };

        self.push_command(
            3,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp).wrapping_add(1),
                    &Register16::Sp,
                );
            }),
        );

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
                Destination::Register(high),
            ),
        );

        self.push_command(
            6,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp).wrapping_add(1),
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
                Destination::Register(low),
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
