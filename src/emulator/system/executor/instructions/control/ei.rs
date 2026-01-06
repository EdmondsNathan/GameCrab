use crate::emulator::{commands::command::Command::Update, system::console::Console};

impl Console {
    pub(super) fn ei(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                console.cpu.set_interrupt(true);
            }),
        );
        Some(4)
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::console::{self, Console};

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn ei() {
        let mut console = init(vec![(0xFB, 0x100)]);

        assert!(!console.cpu.get_interrupt());

        for n in 0..4 {
            console.tick();
        }

        assert!(console.cpu.get_interrupt());
    }
}
