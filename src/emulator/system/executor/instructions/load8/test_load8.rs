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
        components::registers::Register8, executor::instructions::load8::test_load8::init,
    };

    #[test]
    fn to_register_8() {
        let mut console = init(vec![(0x78, 0x100)]);
        console.cpu.set_register(1, &Register8::B);
        console.cpu.set_register(0, &Register8::A);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 1);
    }
}
