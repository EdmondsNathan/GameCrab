use crate::emulator::console::Console;
use crate::emulator::decoder::decode;
use crate::emulator::execution_queue::Command;
use crate::emulator::instruction::Instruction::*;
use crate::emulator::instruction::*;
use crate::emulator::registers::{Register16, Register8};

impl Console {
    pub(super) fn push_command(&mut self, tick_offset: u64, command: Command) {
        self.execution_queue
            .push_command_absolute(self.tick_counter + tick_offset, command);
    }
    pub fn execute(&mut self, instruction: Instruction) {
        self.push_command(1, Command::Standard(Console::command_increment_pc));

        if let Some(next_instruction_offset) = match instruction {
            CB => {
                self.push_command(4, Command::Standard(Console::instruction_cb));
                None
            }
            Control(control_op) => self.instruction_control(control_op),
            Load16(ld16) => self.instruction_load16(ld16),
            Push(push_pop) => todo!(),
            Pop(push_pop) => todo!(),
            Load8(to, from) => todo!(),
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
        let instruction = match decode(self.ram.fetch(self.cpu.get_register_16(Register16::Pc))) {
            Ok(value) => value,
            Err(error) => panic!("{error}"),
        };

        self.execute(instruction);
    }

    pub(crate) fn queue_next_instruction(&mut self, tick: u64) {
        self.push_command(tick, Command::Standard(Console::fetch_decode_execute));
    }

    pub(super) fn command_ram_to_register(&mut self, address: u16, register: Register8) {
        let value = self.ram.fetch(address);

        self.cpu.set_register(value, register);
    }

    pub(super) fn command_register_to_ram(&mut self, address: u16, register: Register8) {
        let value = self.cpu.get_register(register);

        self.ram.set(value, address);
    }

    pub(super) fn command_increment_pc(&mut self) {
        self.cpu
            .set_register_16(self.cpu.get_register_16(Register16::Pc) + 1, Register16::Pc);
    }
}
