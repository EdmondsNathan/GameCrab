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
    pub(crate) fn instruction_restart(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp),
                    &Register16::Bus,
                );
            }),
        );

        self.push_command(
            8,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
                    &Register16::Sp,
                );
            }),
        );

        self.push_command(
            9,
            Read(
                Source::Register(Register8::PcHigh),
                Destination::RamFromRegister(Register16::Bus),
            ),
        );

        self.push_command(
            10,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp),
                    &Register16::Bus,
                );
            }),
        );

        self.push_command(
            11,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
                    &Register16::Sp,
                );
            }),
        );

        // Set xy to pc-1 so we can lookup which restart opcode was called
        self.push_command(
            11,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc) - 1,
                    &Register16::Xy,
                );
            }),
        );

        self.push_command(
            12,
            Read(
                Source::Register(Register8::PcLow),
                Destination::RamFromRegister(Register16::Bus),
            ),
        );

        self.push_command(
            13,
            Update(|console: &mut Console| {
                console.cpu.set_register(0, &Register8::PcHigh);
            }),
        );

        self.push_command(
            14,
            Update(|console: &mut Console| {
                console
                    .cpu
                    .set_register(lookup_restart(console), &Register8::PcLow);
            }),
        );

        Some(16)
    }
}

fn lookup_restart(console: &Console) -> u8 {
    match console
        .ram
        .fetch(console.cpu.get_register_16(&Register16::Xy))
    {
        0xC7 => 0x00,
        0xCF => 0x08,
        0xD7 => 0x10,
        0xDF => 0x18,
        0xE7 => 0x20,
        0xEF => 0x28,
        0xF7 => 0x30,
        0xFF => 0x38,
        _ => panic!(
            "Impossible value 0x{:X}",
            console
                .ram
                .fetch(console.cpu.get_register_16(&Register16::Xy))
        ),
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
    fn restart() {
        let mut console = init(vec![(0xCF, 0x102)]);
        console.cpu.set_register_16(0x102, &Register16::Pc);
        console.cpu.set_register_16(0x201, &Register16::Sp);

        for n in 0..16 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register_16(&Register16::Pc), 0x0008);
        assert_eq!(console.ram.fetch(0x200), 0x03);
        assert_eq!(console.ram.fetch(0x201), 0x01);
    }
}
