use crate::emulator::system::console::Console;

fn init(memory_map: Vec<(u8, u16)>) -> Console {
    let mut console = Console::new();

    for memory in memory_map {
        console.ram.set(memory.0, memory.1);
    }

    console
}

#[cfg(test)]
mod go_from_register_8 {
    use crate::emulator::system::{
        components::registers::{Register16, Register8},
        executor::instructions::load8::test_load8::init,
    };

    #[test]
    fn to_register_8() {
        let mut console = init(vec![(0x78, 0x100)]);
        console.cpu.set_register(1, &Register8::B);
        console.cpu.set_register(0, &Register8::A);

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 1);
    }

    #[test]
    fn to_register_16() {
        let mut console = init(vec![(0x02, 0x100)]);
        console.cpu.set_register(0x10, &Register8::A);
        console.cpu.set_register_16(0x256, &Register16::Bc);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x256), 0x10);
    }

    #[test]
    fn to_hl_ldd() {
        let mut console = init(vec![(0x32, 0x100)]);
        console.cpu.set_register(0x10, &Register8::A);
        console.cpu.set_register_16(0x256, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        // Ensure the byte of ram at Hl's location is loaded with register a's value
        assert_eq!(console.ram.fetch(0x256), 0x10);

        // Ensure hl is decremented
        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x255);
    }

    #[test]
    fn to_hl_ldi() {
        let mut console = init(vec![(0x22, 0x100)]);
        console.cpu.set_register(0x10, &Register8::A);
        console.cpu.set_register_16(0x256, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        // Ensure the byte of ram at Hl's location is loaded with register a's value
        assert_eq!(console.ram.fetch(0x256), 0x10);

        // Ensure hl is incremented
        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x257);
    }

    #[test]
    fn to_u16() {
        // EA is the A to u16 instruction
        // the following 2 bytes are 0x01, 0x02
        // Since it is little endian, this corresponds to address 0x0201
        // This address will be set to the value of register A(0x05)
        let mut console = init(vec![(0xEA, 0x100), (0x01, 0x101), (0x02, 0x102)]);
        console.cpu.set_register(0x05, &Register8::A);

        for n in 0..16 {
            console.tick();
        }

        // Ensure the byte of ram at 0x0201 is loaded with register a's value
        assert_eq!(console.ram.fetch(0x0201), 0x05);
    }
}
