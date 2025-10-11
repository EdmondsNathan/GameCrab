use crate::emulator::{
    commands::command::Command,
    console::Console,
    cpu::CPU,
    executor::instructions::load8::instruction_load8::{Ff00, Hl, To},
    registers::{Register16, Register8},
};
impl Console {
    pub(super) fn go_from_register8(&mut self, to: To, from: Register8) -> Option<u64> {
        fn to_register8(console: &mut Console, to: Register8, from: Register8) -> Option<u64> {
            console.push_command(
                3,
                Command::SetRegister(CPU::set_register, console.cpu.get_register(from), to),
            );
            Some(4)
        }

        fn to_register16(console: &mut Console, to: Register16, from: Register8) -> Option<u64> {
            console.push_command(
                4,
                Command::SetRegister(
                    CPU::set_register,
                    console.ram.fetch(console.cpu.get_register_16(to)),
                    from,
                ),
            );
            Some(8)
        }

        fn to_hl(console: &mut Console, to: Hl, from: Register8) -> Option<u64> {
            todo!();
        }

        fn to_u8(console: &mut Console, from: Register8) -> Option<u64> {
            todo!();
        }

        fn to_u16(console: &mut Console, from: Register8) -> Option<u64> {
            todo!();
        }

        fn to_Ff00(console: &mut Console, to: Ff00, from: Register8) -> Option<u64> {
            todo!();
        }

        match to {
            To::Register8(register8) => to_register8(self, register8, from),
            To::Register16(register16) => to_register16(self, register16, from),
            To::Hl(hl) => to_hl(self, hl, from),
            To::U8 => to_u8(self, from),
            To::U16 => to_u16(self, from),
            To::Ff00(ff00) => to_Ff00(self, ff00, from),
        }
    }
}
