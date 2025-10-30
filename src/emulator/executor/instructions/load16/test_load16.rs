#[cfg(test)]
mod tests {
    use crate::emulator::{console::Console, decoder::decode, registers::Register8};

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn bc_u16() {
        // (1, 0x100) is bc_u16 at address x100
        // the other two are the values to assign to registers C(50) and B(45)
        let mut console = init(vec![(1, 0x100), (50, 0x101), (45, 0x102)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 45);
        assert_eq!(console.cpu.get_register(&Register8::C), 50);
    }

    #[test]
    fn de_u16() {
        // (1, 0x100) is de_u16 at address x100
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
        // (1, 0x100) is hl_u16 at address x100
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
        // (1, 0x100) is sp_u16 at address x100
        // the other two are the values to assign to registers E(50) and D(45)
        let mut console = init(vec![(0x31, 0x100), (50, 0x101), (45, 0x102)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::SpLow), 45);
        assert_eq!(console.cpu.get_register(&Register8::SpHigh), 50);
    }

    #[test]
    fn u16_sp() {
        // (1, 0x100) is de_u16 at address x100
        // the other two are the values to assign to registers E(50) and D(45)
        let mut console = init(vec![(0x31, 0x100), (50, 0x101), (45, 0x102)]);

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::SpLow), 45);
        assert_eq!(console.cpu.get_register(&Register8::SpHigh), 50);
    }
}
