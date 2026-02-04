use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Flags, Register16, Register8},
        console::Console,
        executor::instructions::instruction::Calls,
    },
};

impl Console {
    pub(crate) fn instruction_call(&mut self) -> Option<u64> {
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
            5,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(Register8::Y),
            ),
        );

        self.push_command(
            6,
            Update(|console: &mut Console| {
                console.cpu.set_register_16(
                    console.cpu.get_register_16(&Register16::Pc),
                    &Register16::Bus,
                );
            }),
        );

        self.push_command(7, Update(Console::command_increment_pc));

        self.push_command(
            8,
            Read(
                Source::RamFromRegister(Register16::Bus),
                Destination::Register(Register8::X),
            ),
        );

        self.push_command(
            9,
            Update(|console: &mut Console| {
                let tick_offset = 9;

                if !test_flag(console) {
                    console.queue_next_instruction(12 - tick_offset);
                    return;
                }

                console.push_command(
                    16 - tick_offset,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Sp),
                            &Register16::Bus,
                        );
                    }),
                );

                console.push_command(
                    17 - tick_offset,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
                            &Register16::Sp,
                        );
                    }),
                );

                console.push_command(
                    18 - tick_offset,
                    Read(
                        Source::Register(Register8::PcLow),
                        Destination::RamFromRegister(Register16::Bus),
                    ),
                );

                console.push_command(
                    20 - tick_offset,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Sp),
                            &Register16::Bus,
                        );
                    }),
                );

                console.push_command(
                    21 - tick_offset,
                    Update(|console: &mut Console| {
                        console.cpu.set_register_16(
                            console.cpu.get_register_16(&Register16::Sp).wrapping_sub(1),
                            &Register16::Sp,
                        );
                    }),
                );

                console.push_command(
                    22 - tick_offset,
                    Read(
                        Source::Register(Register8::PcHigh),
                        Destination::RamFromRegister(Register16::Bus),
                    ),
                );

                console.queue_next_instruction(24 - tick_offset);
            }),
        );

        None
    }
}

fn test_flag(console: &Console) -> bool {
    match console
        .ram
        .fetch(console.cpu.get_register_16(&Register16::Bus) - 2)
    {
        0xC4 => !console.cpu.get_flag(&Flags::Z),
        0xCC => console.cpu.get_flag(&Flags::Z),
        0xD4 => !console.cpu.get_flag(&Flags::C),
        0xDC => console.cpu.get_flag(&Flags::C),
        0xCD => true,
        _ => panic!("Impossible value!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::{
        components::registers::{Flags, Register16},
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
    fn call() {
        let mut console = init(vec![(0xC4, 0x100)]);
        console.cpu.set_flag(true, &Flags::Z);
        console.cpu.set_register_16(0x201, &Register16::Sp);

        for n in 0..24 {
            console.tick();
        }

        assert_ne!(console.ram.fetch(0x201), 0x03);
        assert_ne!(console.ram.fetch(0x200), 0x01);

        let mut console = init(vec![(0xCC, 0x100)]);
        console.cpu.set_flag(true, &Flags::Z);
        console.cpu.set_register_16(0x201, &Register16::Sp);

        for n in 0..24 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x201), 0x03);
        assert_eq!(console.ram.fetch(0x200), 0x01);

        let mut console = init(vec![(0xCD, 0x100)]);
        console.cpu.set_flag(true, &Flags::Z);
        console.cpu.set_register_16(0x201, &Register16::Sp);

        for n in 0..24 {
            console.tick();
        }

        assert_eq!(console.ram.fetch(0x201), 0x03);
    }
}
