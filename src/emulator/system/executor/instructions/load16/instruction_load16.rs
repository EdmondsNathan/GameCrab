use crate::emulator::commands::command::{Command::*, Destination, Source};
use crate::emulator::system::{
    components::registers::*, console::Console, executor::instructions::instruction::*,
};

impl Console {
    pub(crate) fn instruction_load16(&mut self, ld16: Ld16) -> Option<u64> {
        match ld16 {
            Ld16::BcU16 => self.u16_to_register(Register16::Bc),
            Ld16::DeU16 => self.u16_to_register(Register16::De),
            Ld16::HlU16 => self.u16_to_register(Register16::Hl),
            Ld16::SpU16 => self.u16_to_register(Register16::Sp),
            Ld16::U16Sp => self.u16sp(),
            Ld16::SpHl => self.sphl(),
        }
    }

    fn u16_to_register(&mut self, register: Register16) -> Option<u64> {
        let (high, low) = register.register16_to_register8();

        self.push_command(
            3,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(4, Update(Self::command_increment_pc));

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
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(7, Update(Self::command_increment_pc));

        self.push_command(
            8,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(high),
            ),
        );

        Some(12)
    }

    fn u16sp(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(4, Update(Self::command_increment_pc));

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
                )
            }),
        );

        self.push_command(7, Update(Self::command_increment_pc));

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
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Xy),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(
            12,
            Read(
                Source::Register(Register8::SpLow),
                Destination::RamFromRegister(Register16::Bus),
            ),
        );

        self.push_command(
            13,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Xy) + 1,
                    &Register16::Xy,
                );
            }),
        );

        self.push_command(
            14,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Xy),
                    &Register16::Bus,
                )
            }),
        );

        self.push_command(
            16,
            Read(
                Source::Register(Register8::SpHigh),
                Destination::RamFromRegister(Register16::Bus),
            ),
        );

        Some(20)
    }

    fn sphl(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Read(
                Source::Register(Register8::H),
                Destination::Register(Register8::SpHigh),
            ),
        );

        self.push_command(
            4,
            Read(
                Source::Register(Register8::L),
                Destination::Register(Register8::SpLow),
            ),
        );

        Some(8)
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
    fn bc_u16() {
        // (0x01, 0x100) is bc_u16 at address 0x100
        // the other two are the values to assign to registers C(50) and B(45)
        let mut console = init(vec![(0x01, 0x100), (50, 0x101), (45, 0x102)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 45);
        assert_eq!(console.cpu.get_register(&Register8::C), 50);
    }

    #[test]
    fn de_u16() {
        // (0x11, 0x100) is de_u16 at address 0x100
        // the other two are the values to assign to registers E(50) and D(45)
        let mut console = init(vec![(0x11, 0x100), (50, 0x101), (45, 0x102)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::D), 45);
        assert_eq!(console.cpu.get_register(&Register8::E), 50);
    }

    #[test]
    fn hl_u16() {
        // (0x21, 0x100) is hl_u16 at address 0x100
        // the other two are the values to assign to registers L(50) and H(45)
        let mut console = init(vec![(0x21, 0x100), (50, 0x101), (45, 0x102)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::H), 45);
        assert_eq!(console.cpu.get_register(&Register8::L), 50);
    }

    #[test]
    fn sp_u16() {
        // (0x31, 0x100) is sp_u16 at address 0x100
        // the other two are the values to assign to registers SpHigh(50) and SpLow(45)
        let mut console = init(vec![(0x31, 0x100), (50, 0x101), (45, 0x102)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::SpLow), 50);
        assert_eq!(console.cpu.get_register(&Register8::SpHigh), 45);
    }

    #[test]
    fn u16_sp() {
        // (0x08, 0x100) is u16_sp at address 0x100
        // the other two are the values to assign to registers SpHigh(08) and SpLow(20)
        let mut console = init(vec![(0x08, 0x100), (0x08, 0x101), (0x20, 0x102)]);
        console.cpu.set_register_16(0x0110, &Register16::Sp);

        for n in 0..20 {
            console.tick();
        }

        assert_eq!(
            console.ram.fetch_16(0x2008),
            console.cpu.get_register_16(&Register16::Sp)
        );
    }

    #[test]
    fn sp_hl() {
        let mut console = init(vec![(0xF9, 0x100)]);
        console.cpu.set_register_16(0x0110, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register_16(&Register16::Sp), 0x0110);
    }
}
