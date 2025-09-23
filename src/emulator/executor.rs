use crate::emulator::console::Console;
use crate::emulator::decoder::{decode, decode_cb};
use crate::emulator::instruction::Instruction::*;
use crate::emulator::instruction::*;
use crate::emulator::registers::Register;

impl Console {
    pub(crate) fn command_fetch_decode_execute(&mut self) {
        let instruction = match self.cpu.cb_mode {
            true => match decode_cb(self.ram.fetch(self.cpu.registers.pc)) {
                Ok(value) => value,
                Err(error) => panic!("{error}"),
            },
            false => match decode(self.ram.fetch(self.cpu.registers.pc)) {
                Ok(value) => value,
                Err(error) => panic!("{error}"),
            },
        };

        self.command_execute(instruction);
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            CB => todo!(),
            Control(control_ops) => todo!(),
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
        }
    }

    fn command_read(&mut self, address: u16, register: Register) {
        let value = self.ram.fetch(address);

        self.cpu.set_register(value, register);
    }

    fn command_write(&mut self, register: Register, address: u16) {
        let value = self.cpu.get_register(register);

        self.ram.set(value, address);
    }

    fn command_execute(&mut self, instruction: Instruction) {
        self.execution_queue
            .push_command(self.tick_counter + 1, |console: &mut Console| {
                console.cpu.registers.pc += 1
            });
        self.run_instruction(instruction);
    }
}
