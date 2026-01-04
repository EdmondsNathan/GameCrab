use crate::emulator::{
    commands::command::Command::Update,
    system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
        executor::instructions::{cb::instruction_cb::CB_OFFSET, instruction::BitArgs},
    },
};

impl Console {
    pub(super) fn rra(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                let register_value = console.cpu.get_register(&Register8::A);
                let carry = register_value & 0b00000001;
                let carry_flag = console.cpu.get_flag(&Flags::C) as u8;

                console
                    .cpu
                    .set_register((register_value >> 1) + (carry_flag << 7), &Register8::A);

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
        components::registers::{Flags, Register16, Register8},
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
    fn rra() {
        let mut console = init(vec![(0x1F, 0x100), (0x1F, 0x101)]);
        console.cpu.set_register(0b10000001, &Register8::A);

        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0b01000000);
        assert_eq!(console.cpu.get_register(&Register8::F), 0b00010000);

        console.cpu.set_register(0b00000000, &Register8::A);
        console.cpu.set_flag(true, &Flags::C);
        for n in 0..4 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register(&Register8::A), 0b10000000);
        assert_eq!(console.cpu.get_register(&Register8::F), 0b00000000);
    }
}
