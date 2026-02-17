use crate::emulator::{
    commands::command::Command::Update,
    system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
        executor::instructions::{cb::instruction_cb::CB_OFFSET, instruction::BitArgs},
    },
};

fn mask(bit: u8) -> u8 {
    let mut value: u8 = 0;
    for n in 0..8 {
        value += ((n != bit) as u8) << n;
    }

    value
}

impl Console {
    pub(super) fn reset(&mut self, bit_args: BitArgs) -> Option<u64> {
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
                7,
                Update(|console: &mut Console| {
                    let ram_value = console
                        .ram
                        .fetch(console.cpu.get_register_16(&Register16::Bus));

                    // Map the instruction range 0x40-0x7F into 8 sections, 0-7
                    let shift: u8 = (console.cpu.get_register(&Register8::Y) - 0x80) / 8;
                    let mask = mask(shift);

                    console.ram.set(
                        ram_value & mask,
                        console.cpu.get_register_16(&Register16::Hl),
                    );
                }),
            );

            Some(16 - CB_OFFSET)
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
                        _ => panic!(
                            "Impossible value {:X}",
                            console.cpu.get_register(&Register8::Y) & 0x0F
                        ),
                    };

                    let register_value = console.cpu.get_register(&register);
                    // Map the instruction range 0x40-0x7F into 8 sections, 0-7
                    let shift: u8 = (console.cpu.get_register(&Register8::Y) - 0x80) / 8;
                    let mask = mask(shift);

                    console.cpu.set_register(register_value & mask, &register);
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
            (0x80, 0x101),
            (0xCB, 0x102),
            (0x89, 0x103),
            (0xCB, 0x104),
            (0x92, 0x105),
            (0xCB, 0x106),
            (0x9B, 0x107),
        ]);
        console.cpu.set_register(0b11111111, &Register8::B);
        console.cpu.set_register(0b11111111, &Register8::C);
        console.cpu.set_register(0b11111111, &Register8::D);
        console.cpu.set_register(0b11111111, &Register8::E);

        // B
        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 0b11111110);

        // C
        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::C), 0b11111101);

        // D
        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::D), 0b11111011);

        // E
        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::E), 0b11110111);
    }

    #[test]
    fn to_hl() {
        let mut console = init(vec![
            (0xCB, 0x100),
            (0x86, 0x101),
            (0b11111111, 0x200),
            (0xCB, 0x102),
            (0x8E, 0x103),
            (0b11111111, 0x201),
            (0xCB, 0x104),
            (0x96, 0x105),
            (0b11111111, 0x202),
        ]);

        console.cpu.set_register_16(0x200, &Register16::Hl);
        for n in 0..16 {
            console.tick();
        }
        assert_eq!(console.ram.fetch(0x200), 0b11111110);

        console.cpu.set_register_16(0x201, &Register16::Hl);
        for n in 0..16 {
            console.tick();
        }
        assert_eq!(console.ram.fetch(0x201), 0b11111101);

        console.cpu.set_register_16(0x202, &Register16::Hl);
        for n in 0..16 {
            console.tick();
        }
        assert_eq!(console.ram.fetch(0x202), 0b11111011);
    }
}
