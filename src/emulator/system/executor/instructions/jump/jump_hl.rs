use crate::emulator::{
    commands::command::Command::Update,
    system::{components::registers::Register16, console::Console},
};

impl Console {
    pub(super) fn hl(&mut self) -> Option<u64> {
        //TAG_TODO
        self.push_command(
            3,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Hl),
                    &Register16::Pc,
                );
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
    fn jump_hl() {
        let mut console = init(vec![(0xE9, 0x100), (0x41, 0x200)]);
        console.cpu.set_register_16(0x200, &Register16::Hl);
        console.cpu.set_register(0x45, &Register8::C);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 0x45);
    }
}
