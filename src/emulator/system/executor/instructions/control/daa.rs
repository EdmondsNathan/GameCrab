// Sourced from https://blog.ollien.com/posts/gb-daa/

use crate::emulator::{
    commands::command::Command::Update,
    system::{
        components::registers::{Flags, Register8},
        console::Console,
    },
};

impl Console {
    pub(super) fn daa(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                let register = console.cpu.get_register(&Register8::A);
                let carry = console.cpu.get_flag(&Flags::C);
                let half_carry = console.cpu.get_flag(&Flags::H);
                let sub = console.cpu.get_flag(&Flags::N);
                let mut offset = 0_u8;
                let mut should_carry = false;

                if (!sub && register & 0x0F > 0x09) || half_carry {
                    offset += 0x06;
                }

                if (!sub && register > 0x99) || carry {
                    offset += 0x60;
                    should_carry = true;
                }

                let result = if sub {
                    register.wrapping_sub(offset)
                } else {
                    register.wrapping_add(offset)
                };

                console.cpu.set_register(result, &Register8::A);

                console.cpu.set_flag(result == 0, &Flags::Z);
                console.cpu.set_flag(false, &Flags::H);
                console.cpu.set_flag(should_carry, &Flags::C);

                // let mut offset: u8 = 0;
                // let (output, carried): (u8, bool) = match sub {
                //     true => {
                //         todo!();
                //     }
                //     false => {
                //         if register & 0x0F > 0x09 || half_carry {
                //             offset += 0x06;
                //         }
                //
                //         if register > 0x99 || carry {
                //             offset += 0x60;
                //         }
                //
                //         register.overflowing_add(offset)
                //     }
                // };
                //
                // console.cpu.set_register(output, &Register8::A);
                //
                // console.cpu.set_flag(output == 0, &Flags::Z);
                // console.cpu.set_flag(false, &Flags::H);
                // console.cpu.set_flag(carried || carry, &Flags::C);
            }),
        );
        Some(4)
    }
}

#[cfg(test)]
mod tests {
    use std::env::join_paths;

    use crate::emulator::system::{
        components::registers::{Flags, Register8},
        console::{self, Console},
    };

    fn init(memory_map: Vec<(u8, u16)>) -> Console {
        let mut console = Console::new();

        for memory in memory_map {
            console.ram.set(memory.0, memory.1);
        }

        console
    }

    fn setup(a: u8, n: bool, h: bool, c: bool) -> Console {
        let mut console = init(vec![(0x27, 0x100)]);
        console.cpu.set_register(a, &Register8::A);
        console.cpu.set_flag(n, &Flags::N);
        console.cpu.set_flag(h, &Flags::H);
        console.cpu.set_flag(c, &Flags::C);

        for n in 0..4 {
            console.tick();
        }

        console
    }

    #[test]
    fn daa_add() {
        // Ensure nothing happens if the number is already BCD
        let console = setup(0x14, false, false, false);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x14);
        assert!(!console.cpu.get_flag(&Flags::C));

        // Ensure the 1s place wraps
        let console = setup(0x7B, false, false, false);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x81);
        assert!(!console.cpu.get_flag(&Flags::C));

        // Ensure the 10s place wraps
        let console = setup(0xA8, false, false, false);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x08);
        assert!(console.cpu.get_flag(&Flags::C));

        // Ensure both wrap
        let console = setup(0xAB, false, false, false);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x11);
        assert!(console.cpu.get_flag(&Flags::C));

        // Ensure the 1s place wraps
        let console = setup(0x7B, false, false, false);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x81);
        assert!(!console.cpu.get_flag(&Flags::C));

        // Ensure carry flag works
        let console = setup(0x10, false, false, true);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x70);
        assert!(console.cpu.get_flag(&Flags::C));

        // Ensure half carry flag works
        let console = setup(0x11, false, true, false);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x17);
        assert!(!console.cpu.get_flag(&Flags::C));
    }

    #[test]
    fn daa_sub() {
        // Ensure nothing happens if the number is already BCD
        let console = setup(0x14, true, false, false);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x14);
        assert!(!console.cpu.get_flag(&Flags::C));

        // Ensure half carry flag works
        let console = setup(0x0D, true, true, false);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x07);
        assert!(!console.cpu.get_flag(&Flags::H));

        // Ensure carry flag works
        let console = setup(0xE4, true, false, true);
        assert_eq!(console.cpu.get_register(&Register8::A), 0x84);
        assert!(console.cpu.get_flag(&Flags::C));
    }
}
