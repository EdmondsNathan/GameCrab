use crate::emulator::commands::command::Command::Update;
use crate::emulator::system::{
    components::registers::{Flags, Register8},
    console::Console,
};

impl Console {
    pub(super) fn rlca(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                let register_value = console.cpu.get_register(&Register8::A);
                let carry = register_value >> 7;

                console
                    .cpu
                    .set_register((register_value << 1) + carry, &Register8::A);

                console.cpu.set_flag(false, &Flags::Z);
                console.cpu.set_flag(false, &Flags::N);
                console.cpu.set_flag(false, &Flags::H);
                console.cpu.set_flag(carry == 1, &Flags::C);
            }),
        );

        Some(4)
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
    fn rlca() {
        let mut console = init(vec![(0x07, 0x100)]);
        console.cpu.set_register(0b10000010, &Register8::A);

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0b00000101);
        assert_eq!(console.cpu.get_register(&Register8::F), 0b00010000);
    }
}
