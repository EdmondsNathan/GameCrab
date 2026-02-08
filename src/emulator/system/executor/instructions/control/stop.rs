use crate::emulator::{
    commands::command::Command::Update,
    system::{components::registers::Register16, console::Console},
};

impl Console {
    pub(super) fn stop(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                );
            }),
        );

        self.push_command(4, Update(Console::command_increment_pc));

        self.push_command(
            7,
            Update(|console: &mut Console| {
                console.cpu.set_is_stopped(true);
            }),
        );

        Some(8)
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
    fn stop() {
        // 0x37 Sets the carry flag to 1, just used to test if the cpu resumed
        let mut console = init(vec![(0x10, 0x100), (0x37, 0x102)]);

        for n in 0..8 {
            console.tick();
        }

        assert!(console.cpu.get_is_stopped());

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.tick_counter, 8);

        // TAG_TODO Test for joypad input
        console.cpu.set_is_stopped(false);

        for n in 0..4 {
            console.tick();
        }

        assert!(console.cpu.get_flag(&Flags::C));
    }
}
