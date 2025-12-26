#[cfg(test)]
mod go_from_u8 {
    use crate::emulator::system::{components::registers::Register8, console::Console};

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn to_register_8() {
        let mut console = init(vec![(0x3E, 0x100), (0x03, 0x101)]);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x03);
    }
}
