use crate::emulator::commands::command::Command;
use crate::emulator::system::executor::instructions::decoder::*;
use crate::emulator::system::{
    components::registers::Register16,
    console,
    console::Console,
    executor::instructions::{
        cb::instruction_cb, control::instruction_control, instruction::Instruction,
        instruction::Instruction::*, load16::instruction_load16, load8::instruction_load8,
    },
};

// use crate::emulator::{
//     commands::command::Command,
//     console::decoder::{decode, decode_cb},
//     console::Console,
//     instruction::{Instruction::*, *},
//     registers::Register16,
// };

impl Console {
    pub(super) fn push_command(&mut self, tick_offset: u64, command: Command) {
        self.execution_queue
            .push_command_absolute(self.tick_counter + tick_offset, command);
    }

    pub fn execute(&mut self, instruction: Instruction) {
        if let Some(next_instruction_offset) = match instruction {
            Cb => self.instruction_cb(),
            Control(control_op) => self.instruction_control(control_op),
            Load16(ld16) => self.instruction_load16(ld16),
            Push(push_pop) => todo!("push not implemented"),
            Pop(push_pop) => todo!("pop not implemented"),
            Load8(to, from) => self.instruction_load8(to, from),
            Arithmetic16(a16_ops) => todo!("arith16 not implemented"),
            Arithmetic8(a8_ops) => todo!("arith8 not implemented"),
            JumpRelative(jr) => todo!("jumpRelative not implemented"),
            Jump(jp) => todo!("jump not implemented"),
            Restart(arg) => todo!("restart not implemented"),
            Return(ret) => todo!("return not implemented"),
            Call(calls) => todo!("call not implemented"),
            BitOp(bit_ops) => todo!("bitOp not implemented"),
        } {
            self.queue_next_instruction(next_instruction_offset);
        }
    }

    pub(crate) fn fetch_decode_execute(&mut self) {
        let decoder = match self.cb_flag {
            true => decode_cb,
            false => decode,
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
