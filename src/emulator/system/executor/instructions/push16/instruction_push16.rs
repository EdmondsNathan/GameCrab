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
    pub(crate) fn stack_push16(&mut self, arg: PushPop) -> Option<u64> {
        let (high, low) = match arg {
            PushPop::Bc => Register16::Bc,
            PushPop::De => Register16::De,
            PushPop::Hl => Register16::Hl,
            PushPop::Af => Register16::Af,
        }
        .register16_to_register8();

        self.push_command(
            8,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp),
                    &Register16::Bus,
                );
            }),
        );

        self.push_command(
            9,
            Read(
                Source::Register(low),
                Destination::RamFromRegister(Register16::Bus),
            ),
        );

        self.push_command(
            10,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp) - 1,
                    &Register16::Sp,
                );
            }),
        );

        self.push_command(
            12,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp),
                    &Register16::Bus,
                );
            }),
        );

        self.push_command(
            13,
            Read(
                Source::Register(high),
                Destination::RamFromRegister(Register16::Bus),
            ),
        );

        self.push_command(
            14,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp) - 1,
                    &Register16::Sp,
                );
            }),
        );

        Some(16)
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{components::registers::Register16, console::Console};

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn stack_push() {
        let mut console = init(vec![(0xC5, 0x100)]);
        console.cpu.set_register_16(0x4567, &Register16::Bc);
        console.cpu.set_register_16(0x201, &Register16::Sp);

        for n in 0..16 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x200), 0x45);
        assert_eq!(console.ram.fetch(0x201), 0x67);
    }
}
