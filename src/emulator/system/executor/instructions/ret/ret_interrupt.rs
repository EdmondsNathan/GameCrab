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
    pub(super) fn ret(&mut self, interrupt: bool) -> Option<u64> {
        let offset = interrupt as u8;

        // M2: Read PClow from [SP], then SP++
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
                Destination::Register(Register8::Y),
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

        // M3: Read PChigh from [SP], then SP++
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
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(Register8::X),
            ),
        );

        self.push_command(
            10,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Sp).wrapping_add(1),
                    &Register16::Sp,
                );
            }),
        );

        // M4: Set PC
        self.push_command(
            11,
            Read(
                Source::Register(Register8::Y),
                Destination::Register(Register8::PcLow),
            ),
        );

        self.push_command(
            12,
            Read(
                Source::Register(Register8::X),
                Destination::Register(Register8::PcHigh),
            ),
        );

        if interrupt {
            self.push_command(
                13,
                Update(|console: &mut Console| {
                    console.cpu.set_ime(true);
                }),
            );
        }

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
    fn ret() {
        // [SP]=PClow=0x20, [SP+1]=PChigh=0x02 → PC=0x0220
        let mut console = init(vec![(0xC9, 0x100), (0x20, 0x200), (0x02, 0x201)]);
        console.cpu.set_register_16(0x200, &Register16::Sp);
        console.cpu.set_ime(false);

        for _n in 0..16 {
            console.tick();
        }

        assert!(!console.cpu.get_ime());
        assert_eq!(console.cpu.get_register_16(&Register16::Pc), 0x0220);
    }

    #[test]
    fn ret_interrupt() {
        // [SP]=PClow=0x20, [SP+1]=PChigh=0x02 → PC=0x0220
        let mut console = init(vec![(0xD9, 0x100), (0x20, 0x200), (0x02, 0x201)]);
        console.cpu.set_register_16(0x200, &Register16::Sp);
        console.cpu.set_ime(false);

        for _n in 0..16 {
            console.tick();
        }

        assert!(console.cpu.get_ime());
        assert_eq!(console.cpu.get_register_16(&Register16::Pc), 0x0220);
    }
}
