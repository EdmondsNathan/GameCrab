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
        let mut console = init(vec![(0x11, 0x100), (50, 0x101), (45, 0x102)]);

        if decode(0x11).is_err() {
            panic!("NOT OK!!!")
        } else {
            println!("OK!!!")
        }

        for n in 0..12 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::D), 45);
        assert_eq!(console.cpu.get_register(&Register8::E), 50);
    }
}
