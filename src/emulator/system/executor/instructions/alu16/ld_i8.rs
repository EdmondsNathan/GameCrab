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
    pub(super) fn ld_i8(&mut self) -> Option<u64> {
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
                console.cpu.set_register(0, &Register8::X);
            }),
        );

        self.push_command(
            7,
            Read(
                Source::Register(Register8::SpHigh),
                Destination::Register(Register8::H),
            ),
        );

        self.push_command(
            8,
            Read(
                Source::Register(Register8::SpLow),
                Destination::Register(Register8::L),
            ),
        );

        self.push_command(
            9,
            Update(|console: &mut Console| {
                let l_register = console.cpu.get_register(&Register8::L);
                let offset = console.cpu.get_register(&Register8::Y);
                let (_, carry) = l_register.overflowing_add(offset);
                let half_carry = (l_register & 0x0F) + (offset & 0x0F) > 0x0F;

                console.cpu.set_flag(false, &Flags::Z);
                console.cpu.set_flag(false, &Flags::N);
                console.cpu.set_flag(half_carry, &Flags::H);
                console.cpu.set_flag(carry, &Flags::C);
            }),
        );

        self.push_command(
            10,
            Update(|console: &mut Console| {
                let hl = console.cpu.get_register_16(&Register16::Hl);
                let offset = console.cpu.get_register(&Register8::Y) as i8 as i16;
                let result = (hl as i16).wrapping_add(offset) as u16;

                console.cpu.set_register_16(result, &Register16::Hl);
            }),
        );

        Some(12)
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{
        components::registers::{Flags, Register16},
        console::Console,
    };

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    fn test(sp: u16, offset: u8, output: u16, h_flag: bool, c_flag: bool) {
        let mut console = init(vec![(0xF8, 0x100), (offset, 0x101)]);
        console.cpu.set_register_16(sp, &Register16::Sp);

        for n in 0..12 {
            console.tick();
        }

        assert!(console.cpu.get_register_16(&Register16::Hl) == output);
        assert_eq!(console.cpu.get_flag(&Flags::H), h_flag);
        assert_eq!(console.cpu.get_flag(&Flags::C), c_flag);
    }

    #[test]
    fn ld_i8() {
        test(0xFFF8, 0x08, 0x0000, true, true);
        test(0x0001, 0xFF, 0x0000, true, true);
        test(0x00FF, 0x01, 0x0100, true, true);
        test(0x1234, 0x0C, 0x1240, true, false);
    }
}
