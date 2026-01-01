use crate::emulator::{
    commands::command::Command::Update,
    system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
        executor::instructions::{cb::instruction_cb::CB_OFFSET, instruction::BitArgs},
    },
};

impl Console {
    pub(super) fn rlc(&mut self, bit_args: BitArgs) -> Option<u64> {
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

                    let carry = ram_value >> 7;

                    console.ram.set(
                        (ram_value << 1) + carry,
                        console.cpu.get_register_16(&Register16::Bus),
                    );

                    console.cpu.set_flag(ram_value == 0, &Flags::Z);
                    console.cpu.set_flag(false, &Flags::N);
                    console.cpu.set_flag(false, &Flags::H);
                    console.cpu.set_flag(carry == 1, &Flags::C);
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
                        _ => panic!("Impossible value"),
                    };

                    let register_value = console.cpu.get_register(&register);
                    let carry = register_value >> 7;

                    console
                        .cpu
                        .set_register((register_value << 1) + carry, &register);

                    console.cpu.set_flag(register_value == 0, &Flags::Z);
                    console.cpu.set_flag(false, &Flags::N);
                    console.cpu.set_flag(false, &Flags::H);
                    console.cpu.set_flag(carry == 1, &Flags::C);
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
    fn to_register8() {
        let mut console = init(vec![
            (0xCB, 0x100),
            (0x00, 0x101),
            (0xCB, 0x102),
            (0x01, 0x103),
            (0xCB, 0x104),
            (0x02, 0x105),
        ]);
        console.cpu.set_register(0b10000010, &Register8::B);
        console.cpu.set_register(0b00000000, &Register8::C);
        console.cpu.set_register(0b01000000, &Register8::D);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 0b00000101);
        assert_eq!(console.cpu.get_register(&Register8::F), 0b00010000);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::C), 0b00000000);
        assert_eq!(console.cpu.get_register(&Register8::F), 0b10000000);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::D), 0b10000000);
        assert_eq!(console.cpu.get_register(&Register8::F), 0b00000000);
    }

    #[test]
    fn to_hl() {
        let mut console = init(vec![
            (0xCB, 0x100),
            (0x06, 0x101),
            (0b10000010, 0x200),
            (0xCB, 0x102),
            (0x06, 0x103),
            (0b00000000, 0x201),
        ]);
        console.cpu.set_register_16(0x200, &Register16::Hl);

        for n in 0..16 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x200), 0b00000101);
        assert_eq!(console.cpu.get_register(&Register8::F), 0b00010000);

        console.cpu.set_register_16(0x201, &Register16::Hl);

        for n in 0..16 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x201), 0b00000000);
        assert_eq!(console.cpu.get_register(&Register8::F), 0b10000000);
    }
}
