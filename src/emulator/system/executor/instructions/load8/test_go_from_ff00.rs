#[cfg(test)]
mod go_from_ff00 {
    use crate::emulator::system::{components::registers::Register8, console::Console};

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn plus_c() {
        let mut console = init(vec![(0xF2, 0x100), (0x02, 0xFF01)]);
        console.cpu.set_register(0x01, &Register8::C);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x02);
    }

    #[test]
    fn plus_u8() {
        let mut console = init(vec![(0xF0, 0x100), (0x01, 0x101), (0x02, 0xFF01)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x02);
    }
}
