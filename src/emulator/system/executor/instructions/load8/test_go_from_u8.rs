#[cfg(test)]
mod go_from_u8 {
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
    fn to_register_8() {
        let mut console = init(vec![(0x3E, 0x100), (0x03, 0x101)]);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x03);
    }

    fn to_register_16() {
        let mut console = init(vec![(0x36, 0x100), (0x03, 0x101)]);
        console.cpu.set_register_16(0x102, &Register16::Hl);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x102), 0x03);
    }
}
