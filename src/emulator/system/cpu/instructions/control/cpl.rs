use crate::emulator::{
    commands::command::Command::Update,
    system::{
        components::registers::{Flags, Register8},
        console::Console,
    },
};

impl Console {
    pub(super) fn cpl(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                let register = console.cpu.get_register(&Register8::A);
                console.cpu.set_register(!register, &Register8::A);

                console.cpu.set_flag(true, &Flags::N);
                console.cpu.set_flag(true, &Flags::H);
            }),
        );

        Some(4)
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{
        components::registers::{Flags, Register8},
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
    fn cpl() {
        let mut console = init(vec![(0x2F, 0x100)]);
        console.cpu.set_register(0b11001100, &Register8::A);

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0b00110011);
    }
}
