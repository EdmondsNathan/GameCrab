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
            instructions::{cb::instruction_cb, decoder::decode_cb, instruction::Instruction},
        },
    },
};

pub(crate) const CB_OFFSET: u64 = 5;

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

        // Run at tick 5 so the bitop instructions
        // can be scheduled for the next tick
        self.push_command(
            5,
            Update(|console: &mut Console| {
                if let Ok(Instruction::BitOp(bit_op)) =
                    decode_cb(console.cpu.get_register(&Register8::Y))
                    && let Some(next_instruction_offset) = console.instruction_bit_op(bit_op)
                {
                    console.queue_next_instruction(next_instruction_offset);
                }
            }),
        );

        // The next instruction is queued in the Update command above,
        // so there is no need to queue it here
        None
    }
}
