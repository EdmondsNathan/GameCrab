use crate::emulator::{
    commands::command::Command::Update,
    system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
    },
};

impl Console {
    pub(super) fn add16(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                let (_, low) = lookup_register(console).register16_to_register8();
                let add_register = console.cpu.get_register(&low);
                let l_register = console.cpu.get_register(&Register8::L);
                let (result, low_carry) = l_register.overflowing_add(add_register);

                console.cpu.set_register(result, &Register8::L);

                console.cpu.set_flag(low_carry, &Flags::H);
            }),
        );

        self.push_command(
            4,
            Update(|console: &mut Console| {
                let (high, low) = lookup_register(console).register16_to_register8();
                let add_register = console.cpu.get_register(&high);
                let h_register = console.cpu.get_register(&Register8::H);
                let low_carry = console.cpu.get_flag(&Flags::H);
                let half_carry = (h_register & 0x0F) + (add_register & 0x0F) > 0x0F;
                let (result, carry1) = h_register.overflowing_add(add_register);
                let (result, carry2) = result.overflowing_add(low_carry.into());
                let carry = carry1 || carry2;

                console.cpu.set_register(result, &Register8::H);

                console.cpu.set_flag(half_carry, &Flags::H);
                console.cpu.set_flag(false, &Flags::N);
                console.cpu.set_flag(carry, &Flags::C);
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
        0x09 => Register16::Bc,
        0x19 => Register16::De,
        0x29 => Register16::Hl,
        0x39 => Register16::Sp,
        _ => panic!("Invalid state!"),
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
    fn add16() {
        let mut console = init(vec![(0x09, 0x100), (0x19, 0x101)]);
        console.cpu.set_register_16(0x00FF, &Register16::Bc);
        console.cpu.set_register_16(0x00FF, &Register16::Hl);

        for n in 0..8 {
            console.tick();
        }

        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x01FE);
        assert!(console.cpu.get_flag(&Flags::H));

        console.cpu.set_register_16(0x0001, &Register16::Hl);
        console.cpu.set_register_16(0xFFFF, &Register16::De);

        for n in 0..8 {
            console.tick();
        }
        println!("HL: {}", console.cpu.get_register_16(&Register16::Hl));
        println!("DE: {}", console.cpu.get_register_16(&Register16::De));

        assert_eq!(console.cpu.get_register_16(&Register16::Hl), 0x0000);
        assert!(console.cpu.get_flag(&Flags::H));
        assert!(console.cpu.get_flag(&Flags::C));
    }
}
