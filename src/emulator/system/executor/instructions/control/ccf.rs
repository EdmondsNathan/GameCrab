use crate::emulator::{
    commands::command::Command::Update,
    system::{components::registers::Flags, console::Console},
};

impl Console {
    pub(super) fn ccf(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                let carry = console.cpu.get_flag(&Flags::C);

                console.cpu.set_flag(false, &Flags::N);
                console.cpu.set_flag(false, &Flags::H);
                console.cpu.set_flag(!carry, &Flags::C);
            }),
        );

        Some(4)
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{components::registers::Flags, console::Console};

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn ccf() {
        let mut console = init(vec![(0x3F, 0x100), (0x3F, 0x101)]);
        console.cpu.set_flag(false, &Flags::C);

        for n in 0..4 {
            console.tick();
        }

        assert!(console.cpu.get_flag(&Flags::C));

        for n in 0..4 {
            console.tick();
        }

        assert!(!console.cpu.get_flag(&Flags::C));
    }
}
