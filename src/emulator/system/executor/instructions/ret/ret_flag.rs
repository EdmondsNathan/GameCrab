use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
    },
};

impl Console {
    pub(super) fn ret_flag(&mut self) -> Option<u64> {
        self.push_command(
            3,
            Update(|console: &mut Console| {
                // All the instructions are queued inside of this one
                // so we can subtract 3 from each tick to account for this
                let tick_offset = 3;

                if !test_flag(console) {
                    console.queue_next_instruction(8 - tick_offset);
                    return;
                } // From this point on, our test flag result succeeded

                console.push_command(
                    4 - tick_offset,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Sp).wrapping_add(1),
                            &Register16::Sp,
                        );
                    }),
                );

                console.push_command(
                    5 - tick_offset,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Sp),
                            &Register16::Bus,
                        );
                    }),
                );

                console.push_command(
                    6 - tick_offset,
                    Read(
                        Source::RamFromRegister(Register16::Bus),
                        Destination::Register(Register8::Y),
                    ),
                );

                console.push_command(
                    7 - tick_offset,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Sp).wrapping_add(1),
                            &Register16::Sp,
                        );
                    }),
                );

                console.push_command(
                    8 - tick_offset,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Sp),
                            &Register16::Bus,
                        );
                    }),
                );

                console.push_command(
                    9 - tick_offset,
                    Read(
                        Source::RamFromRegister(Register16::Bus),
                        Destination::Register(Register8::X),
                    ),
                );

                console.push_command(
                    10,
                    Read(
                        Source::Register(Register8::X),
                        Destination::Register(Register8::PcHigh),
                    ),
                );

                console.push_command(
                    11,
                    Read(
                        Source::Register(Register8::Y),
                        Destination::Register(Register8::PcLow),
                    ),
                );

                console.queue_next_instruction(20 - tick_offset);
            }),
        );

        None
    }
}

fn test_flag(console: &Console) -> bool {
    match console
        .ram
        .fetch(console.cpu.get_register_16(&Register16::Bus))
    {
        0xC0 => !console.cpu.get_flag(&Flags::Z),
        0xC8 => console.cpu.get_flag(&Flags::Z),
        0xD0 => !console.cpu.get_flag(&Flags::C),
        0xD8 => console.cpu.get_flag(&Flags::C),
        _ => panic!("Impossible value!"),
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
    fn ret_flag() {
        let mut console = init(vec![(0xC0, 0x100), (0x20, 0x201), (0x02, 0x202)]);
        console.cpu.set_register_16(0x200, &Register16::Sp);
        console.cpu.set_flag(
            true,
            &crate::emulator::system::components::registers::Flags::Z,
        );

        for n in 0..16 {
            console.tick();
        }

        assert_ne!(console.cpu.get_register_16(&Register16::Pc), 0x0220);

        let mut console = init(vec![(0xD8, 0x100), (0x20, 0x201), (0x02, 0x202)]);
        console.cpu.set_register_16(0x200, &Register16::Sp);
        console.cpu.set_flag(
            true,
            &crate::emulator::system::components::registers::Flags::C,
        );

        for n in 0..16 {
            console.tick();
        }

        assert!(!console.cpu.get_ime());
        assert_eq!(console.cpu.get_register_16(&Register16::Pc), 0x0220);
    }
}
