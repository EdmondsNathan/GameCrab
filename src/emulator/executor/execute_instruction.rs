use crate::emulator::{
    commands::command::Command,
    console::Console,
    decoder::{decode, decode_cb},
    instruction::{Instruction::*, *},
    registers::Register16,
};

impl Console {
    pub(super) fn push_command(&mut self, tick_offset: u64, command: Command) {
        self.execution_queue
            .push_command_absolute(self.tick_counter + tick_offset, command);
    }
    pub fn execute(&mut self, instruction: Instruction) {
        if let Some(next_instruction_offset) = match instruction {
            CB => self.instruction_cb(),
            Control(control_op) => self.instruction_control(control_op),
            Load16(ld16) => self.instruction_load16(ld16),
            Push(push_pop) => todo!(),
            Pop(push_pop) => todo!(),
            Load8(to, from) => self.instruction_load8(to, from),
            Arithmetic16(a16_ops) => todo!(),
            Arithmetic8(a8_ops) => todo!(),
            JumpRelative(jr) => todo!(),
            Jump(jp) => todo!(),
            Restart(arg) => todo!(),
            Return(ret) => todo!(),
            Call(calls) => todo!(),
            BitOp(bit_ops) => todo!(),
        } {
            self.queue_next_instruction(next_instruction_offset);
        }
    }

    fn fetch_decode_execute(&mut self) {
        let decoder = match self.cb_flag {
            true => decode,
            false => decode_cb,
        };

        let instruction = match decoder(self.ram.fetch(self.cpu.get_register_16(&Register16::Pc))) {
            Ok(value) => value,
            Err(error) => panic!("{error}"),
        };

        self.cb_flag = false;

        self.push_command(1, Command::Update(Self::command_increment_pc));

        self.execute(instruction);
    }

    pub(crate) fn queue_next_instruction(&mut self, tick: u64) {
        self.push_command(tick, Command::Update(Console::fetch_decode_execute));
    }
}
