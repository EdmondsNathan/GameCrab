use crate::emulator::commands::command::{Command, Command::Update};
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

impl Console {
    /// Push a command onto the queue at the current tick + tick offset.
    pub(super) fn push_command(&mut self, tick_offset: u64, command: Command) {
        self.execution_queue
            .push_command_absolute(self.tick_counter + tick_offset, command);
    }

    /// Queue an instruction.
    pub fn execute(&mut self, instruction: Instruction) {
        let ime_pending = self.cpu.get_ime_pending();

        // Execute an instruction
        // Queue the next instruction at the offset if one is returned
        if let Some(next_instruction_offset) = match instruction {
            Cb => self.instruction_cb(),
            Control(control_op) => self.instruction_control(control_op),
            Load16(ld16) => self.instruction_load16(ld16),
            Push(push_pop) => self.stack_push16(push_pop),
            Pop(push_pop) => self.stack_pop16(push_pop),
            Load8(to, from) => self.instruction_load8(to, from),
            Arithmetic16(a16_ops) => self.instruction_alu16(a16_ops),
            Arithmetic8(a8_ops) => self.instruction_alu8(a8_ops),
            JumpRelative(jr) => self.instruction_jr(jr),
            Jump(jp) => self.instruction_jump(jp),
            Restart(arg) => self.instruction_restart(),
            Return(ret) => self.instruction_ret(ret),
            Call(calls) => self.instruction_call(),
            BitOp(bit_ops) => self.instruction_bit_op(bit_ops),
        } {
            if ime_pending {
                self.push_command(
                    next_instruction_offset - 1,
                    Update(|console: &mut Console| {
                        console.cpu.set_ime(true);
                    }),
                );
            }
            self.queue_next_instruction(next_instruction_offset);
        }
    }

    // TAG_REFACTOR Split into separate functions to increase readability.
    /// Fetch an instruction at the address of PC and then queue that instruction.
    pub(crate) fn fetch_decode_execute(&mut self) {
        // println!(
        //     "fetching PC: {:x} RAM: {:x}",
        //     self.cpu.get_register_16(&Register16::Pc),
        //     self.ram.fetch(self.cpu.get_register_16(&Register16::Pc))
        // );
        let decoder = match self.cb_flag {
            true => decode_cb,
            false => decode,
        };

        let instruction = match decoder(self.ram.fetch(self.cpu.get_register_16(&Register16::Pc))) {
            Ok(value) => value,
            Err(error) => panic!("{error}"),
        };

        self.cb_flag = false;

        self.cpu
            .set_register_16(self.cpu.get_register_16(&Register16::Pc), &Register16::Bus);

        // TAG_REFACTOR remove the halt bug check and handle it elsewhere
        // Every instruction increments the pc after 1 tick
        // unless the halt bug has occured, in which case it is skipped.
        if !self.cpu.get_halt_bug() {
            self.push_command(1, Command::Update(Self::command_increment_pc));
        }

        self.execute(instruction);
    }

    /// Queue the next instruction at the specified tick offset.
    pub(crate) fn queue_next_instruction(&mut self, tick: u64) {
        self.push_command(tick, Command::Update(Console::fetch_decode_execute));
    }
}
