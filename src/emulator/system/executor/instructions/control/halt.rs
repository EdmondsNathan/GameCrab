use crate::emulator::{commands::command::Command::Update, system::console::Console};

impl Console {
    pub(super) fn halt(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                let interrupt_enable = console.ram.fetch(0xFFFF);
                let interrupt_flag = console.ram.fetch(0xFF0F);
                let halt_bug = console.cpu.get_ime() && ((interrupt_enable & interrupt_flag) != 0);

                console.cpu.set_halt(true);
                console.cpu.set_halt_bug(halt_bug);
            }),
        );

        None
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

    fn setup(ime: bool, int_e: u8, int_f: u8) -> Console {
        let mut console = init(vec![(0x76, 0x100)]);
        console.cpu.set_ime(ime);
        console.ram.set(int_e, 0xFFFF);
        console.ram.set(int_f, 0xFF0F);

        console
    }

    #[test]
    fn halt_no_ime() {
        let mut console = setup(false, 0b0000000, 0b00000001);
        // LD B, C
        console.ram.set(0x41, 0x101);
        console.cpu.set_register(0x45, &Register8::C);

        // Halt and do not un-halt, no IME and IE & IF = 0
        for n in 0..8 {
            console.tick();
        }

        assert_ne!(console.cpu.get_register(&Register8::B), 0x45);

        // IE & IF != 0, so queue a new instruction on the next tick
        console.ram.set(0b00000001, 0xFFFF);
        for n in 0..4 {
            console.tick();
        }

        assert_ne!(console.cpu.get_register(&Register8::B), 0x45);

        // The LD B, C instruction can now run
        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 0x45);
    }

    fn halt_bug() {
        let mut console = setup(false, 0b0000001, 0b00000001);
        // LD B, C
        console.ram.set(0x41, 0x101);
        console.cpu.set_register(0x45, &Register8::C);

        // Halt bug, so next command should run immediately
        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 0x45);

        // PC should not increment, run previous command again
        console.cpu.set_register(0x40, &Register8::C);
        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::B), 0x40);
    }

    fn halt_ime() {
        //TAG_TODO
        todo!()
    }
}
