#[cfg(test)]
mod go_from_hl {
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
    fn hl_plus() {
        let mut console = init(vec![(0x2A, 0x100), (0x05, 0x103)]);
        console.cpu.set_register_16(0x103, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x05);
        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x104);
    }

    #[test]
    fn hl_minus() {
        let mut console = init(vec![(0x3A, 0x100), (0x05, 0x103)]);
        console.cpu.set_register_16(0x103, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x05);
        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x102);
    }
}
