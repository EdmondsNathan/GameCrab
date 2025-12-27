use crate::emulator::system::console::Console;

fn init(memory_map: Vec<(u8, u16)>) -> Console {
    let mut console = Console::new();

    for memory in memory_map {
        console.ram.set(memory.0, memory.1);
    }

    console
}

#[cfg(test)]
mod go_from_register_16 {
    use crate::emulator::system::{
        components::registers::{Register16, Register8},
        executor::instructions::load8::test_go_from_register16::init,
    };

    #[test]
    fn to_register_8() {
        let mut console = init(vec![(0x0A, 0x100), (0x03, 0x256)]);
        console.cpu.set_register_16(0x256, &Register16::Bc);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0x03);
    }
}
