use crate::emulator::{
    commands::command::Command::Update,
    system::{
        components::registers::Register16, console::Console,
        executor::instructions::instruction::A16Args,
    },
};

impl Console {
    pub(super) fn inc16(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                let (_, low) = lookup_register(console).register16_to_register8();
                let value = console.cpu.get_register(&low).wrapping_add(1);

                console.cpu.set_register(value, &low);
            }),
        );

        self.push_command(
            4,
            Update(|console: &mut Console| {
                let (high, low) = lookup_register(console).register16_to_register8();
                let carry = (console.cpu.get_register(&low) == 0) as u8;
                let value = console.cpu.get_register(&high).wrapping_add(carry);

                console.cpu.set_register(value, &high);
            }),
        );

        Some(8)
    }
}

fn lookup_register(console: &Console) -> Register16 {
    match console
        .ram
        .fetch(console.cpu.get_register_16(&Register16::Bus))
    {
        0x03 => Register16::Bc,
        0x13 => Register16::De,
        0x23 => Register16::Hl,
        0x33 => Register16::Sp,
        _ => panic!("Invalid state!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{
        components::registers::Register16,
        console::{self, Console},
    };

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    #[test]
    fn inc16() {
        let mut console = init(vec![(0x03, 0x100)]);
        console.cpu.set_register_16(0xFFFF, &Register16::Bc);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register_16(&Register16::Bc), 0x0000);
    }
}
