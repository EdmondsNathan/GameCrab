use crate::emulator::console::Console;
use crate::emulator::decoder::decode_cb;
use crate::emulator::execution_queue::Command;
use crate::emulator::instruction::Instruction::*;
use crate::emulator::instruction::*;
use crate::emulator::registers::{Register16, Register8};

impl Console {
    fn queue_next_instruction(&mut self, tick: u64) {
        self.execution_queue
            .push_command(tick, Command::Standard(Console::fetch_decode_execute));
    }

    pub(super) fn execute(&mut self, instruction: Instruction) {
        self.execution_queue.push_command(
            self.tick_counter + 1,
            Command::Standard(Console::command_increment_pc),
        );

        if let Some(next_instruction_offset) = match instruction {
            CB => {
                self.execution_queue.push_command(
                    self.tick_counter + 4,
                    Command::Standard(Console::instruction_cb),
                );
                None
            }
            Control(control_op) => {
                self.instruction_control(control_op);
            }
            Load16(ld16) => todo!(),
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
            self.queue_next_instruction(self.tick_counter + 4);
        }
    }

    fn instruction_cb(&mut self) {
        let instruction = match decode_cb(self.ram.fetch(self.cpu.get_register_16(Register16::Pc)))
        {
            Ok(value) => value,
            Err(error) => panic!("{error}"),
        };

        self.execute(instruction);
    }

    fn instruction_control(&mut self, control_op: ControlOps) -> Option<u8> {
        match control_op {
            ControlOps::NOP => Some(4),
            ControlOps::STOP => todo!(),
            ControlOps::HALT => todo!(),
            ControlOps::DI => {
                self.execution_queue.push_command(
                    self.tick_counter + 3,
                    Command::Standard(|console: &mut Console| {
                        console.cpu.enable_interrupts = false;
                    }),
                );
                Some(4)
            }
            ControlOps::EI => {
                self.execution_queue.push_command(
                    self.tick_counter + 3,
                    Command::Standard(|console: &mut Console| {
                        console.cpu.enable_interrupts = false;
                    }),
                );
                Some(4)
            }
            ControlOps::DAA => todo!(),
            ControlOps::SCF => todo!(),
            ControlOps::CPL => todo!(),
            ControlOps::CCF => todo!(),
        }
    }

    fn instruction_load16(&mut self, ld16: Ld16) -> Option<u8> {
        match ld16 {
            Ld16::BCU16 => {}
            Ld16::DEU16 => todo!(),
            Ld16::HLU16 => todo!(),
            Ld16::SPU16 => todo!(),
            Ld16::U16SP => todo!(),
            Ld16::SPHL => todo!(),
        }
    }

    fn instruction_load8(&mut self, ld8: Ld8) {
        match ld8 {
            Ld8::A => todo!(),
            Ld8::B => todo!(),
            Ld8::C => todo!(),
            Ld8::D => todo!(),
            Ld8::E => todo!(),
            Ld8::H => todo!(),
            Ld8::L => todo!(),
            Ld8::HL => todo!(),
            Ld8::HLPlus => todo!(),
            Ld8::HLMinus => todo!(),
            Ld8::BC => todo!(),
            Ld8::DE => todo!(),
            Ld8::U16 => todo!(),
            Ld8::U8 => todo!(),
            Ld8::FF00AddU8 => todo!(),
            Ld8::FF00AddC => todo!(),
        }
    }
}
