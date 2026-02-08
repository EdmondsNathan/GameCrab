use crate::emulator::{
    commands::command::Command::Update,
    system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
        executor::instructions::{cb::instruction_cb::CB_OFFSET, instruction::BitArgs},
    },
};

impl Console {
    pub(super) fn bit(&mut self, bit_args: BitArgs) -> Option<u64> {
        fn to_hl(console: &mut Console) -> Option<u64> {
            console.push_command(
                3,
                Update(|console: &mut Console| {
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Hl),
                        &Register16::Bus,
                    );
                }),
            );

            console.push_command(
                6,
                Update(|console: &mut Console| {
                    let ram_value = console
                        .ram
                        .fetch(console.cpu.get_register_16(&Register16::Bus));

                    // Map the instruction range 0x40-0x7F into 8 sections, 0-7
                    let shift: u8 = (console.cpu.get_register(&Register8::Y) - 0x40) / 8;
                    // Shift the bit we're interested in into the 1's place and then mask it out
                    let bit = (ram_value >> shift) & 0b00000001;

                    // The Z bit is set to the OPPOSITE of the bit
                    console.cpu.set_flag(bit == 0, &Flags::Z);
                    console.cpu.set_flag(false, &Flags::N);
                    console.cpu.set_flag(true, &Flags::H);
                }),
            );

            Some(12 - CB_OFFSET)
        }

        fn to_register8(console: &mut Console, register: Register8) -> Option<u64> {
            console.push_command(
                1,
                Update(|console: &mut Console| {
                    let register = match console.cpu.get_register(&Register8::Y) & 0x0F {
                        0x00 => Register8::B,
                        0x01 => Register8::C,
                        0x02 => Register8::D,
                        0x03 => Register8::E,
                        0x04 => Register8::H,
                        0x05 => Register8::L,
                        0x07 => Register8::A,
                        0x08 => Register8::B,
                        0x09 => Register8::C,
                        0x0A => Register8::D,
                        0x0B => Register8::E,
                        0x0C => Register8::H,
                        0x0D => Register8::L,
                        0x0F => Register8::A,
                        _ => panic!("Impossible value"),
                    };

                    let register_value = console.cpu.get_register(&register);
                    // Map the instruction range 0x40-0x7F into 8 sections, 0-7
                    let shift: u8 = (console.cpu.get_register(&Register8::Y) - 0x40) / 8;
                    // Shift the bit we're interested in into the 1's place and then mask it out
                    let bit = (register_value >> shift) & 0b00000001;

                    // The Z bit is set to the OPPOSITE of the bit
                    console.cpu.set_flag(bit == 0, &Flags::Z);
                    console.cpu.set_flag(false, &Flags::N);
                    console.cpu.set_flag(true, &Flags::H);
                }),
            );
            Some(8 - CB_OFFSET)
        }

        match bit_args {
            BitArgs::HL => to_hl(self),
            BitArgs::Register(register) => to_register8(self, register),
        }
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
    fn to_register8() {
        let mut console = init(vec![
            (0xCB, 0x100),
            (0x40, 0x101),
            (0xCB, 0x102),
            (0x41, 0x103),
            (0xCB, 0x104),
            (0x7A, 0x105),
            (0xCB, 0x106),
            (0x7B, 0x107),
        ]);
        console.cpu.set_register(0b00000000, &Register8::B);
        console.cpu.set_register(0b11111111, &Register8::C);
        console.cpu.set_register(0b10001000, &Register8::D);
        console.cpu.set_register(0b00000010, &Register8::E);

        // B
        for n in 0..8 {
            console.tick();
        }

        assert!(console.cpu.get_flag(&Flags::Z));

        // C
        for n in 0..8 {
            console.tick();
        }

        assert!(!console.cpu.get_flag(&Flags::Z));

        // D
        for n in 0..8 {
            console.tick();
        }

        assert!(!console.cpu.get_flag(&Flags::Z));

        // E
        for n in 0..8 {
            console.tick();
        }

        assert!(console.cpu.get_flag(&Flags::Z));
    }

    #[test]
    fn to_hl() {
        let mut console = init(vec![
            (0xCB, 0x100),
            (0x46, 0x101),
            (0b11111110, 0x200),
            (0xCB, 0x102),
            (0x46, 0x103),
            (0b00000001, 0x201),
            (0xCB, 0x104),
            (0x7E, 0x105),
            (0b01111111, 0x202),
        ]);

        console.cpu.set_register_16(0x200, &Register16::Hl);
        for n in 0..12 {
            console.tick();
        }
        assert!(console.cpu.get_flag(&Flags::Z));

        console.cpu.set_register_16(0x201, &Register16::Hl);
        for n in 0..12 {
            console.tick();
        }
        assert!(!console.cpu.get_flag(&Flags::Z));

        console.cpu.set_register_16(0x202, &Register16::Hl);
        for n in 0..12 {
            console.tick();
        }
        assert!(console.cpu.get_flag(&Flags::Z));
    }
}
