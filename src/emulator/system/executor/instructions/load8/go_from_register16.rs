use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Register16, Register8},
        console::Console,
        executor::instructions::load8::instruction_load8::{Ff00, Hl, To},
    },
};

impl Console {
    pub(super) fn go_from_register16(&mut self, to: To, from: Register16) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: Register16) -> Option<u64> {
            // Push from value into xy so we can use the value inside of closure
            console
                .cpu
                .set_register_16(console.cpu.get_register_16(&from), &Register16::Xy);

            console.push_command(
                3,
                Update(|console: &mut Console| {
                    console.cpu.set_register_16(
                        console.cpu.get_register_16(&Register16::Xy),
                        &Register16::Bus,
                    );
                }),
            );

            console.push_command(
                4,
                Read(
                    Source::RamFromRegister(Register16::Bus),
                    Destination::Register(to),
                ),
            );

            Some(8)
        }

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            To::Register16(register16) => panic!("Invalid instruction!"),
            To::Hl(hl) => panic!("Invalid instruction!"),
            To::U8 => panic!("Invalid instruction!"),
            To::U16 => panic!("Invalid instruction!"),
            To::Ff00(ff00) => panic!("Invalid instruction!"),
        }
    }
}
