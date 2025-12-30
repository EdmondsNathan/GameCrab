use crate::emulator::{
    commands::command::{
        Command::{Read, Update},
        Destination, Source,
    },
    system::{
        components::registers::{Register16, Register8},
        console::Console,
        executor::{
            self,
            instructions::{cb::instruction_cb, decoder::decode_cb},
        },
    },
};

//TAG_TODO
impl Console {
    pub(crate) fn instruction_cb(&mut self) -> Option<u64> {
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
                //TAG_TODO Run cb instruction
                if let Ok(instruction) = decode_cb(console.cpu.get_register(&Register8::Y)) {
                    console.execute(instruction);
                }
            }),
        );

        // The CB command returns Some(8), so the CB commands should all return None
        Some(8)
    }
}
