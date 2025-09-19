use crate::emulator::cpu::CPU;
use crate::emulator::instruction::Instruction::*;
use crate::emulator::instruction::*;

pub(crate) fn execute(cpu: &mut CPU, instruction: Instruction) {
    match instruction {
        CB => {
            cb_instruction(cpu);
        }
        Control(control) => {
            control_instruction(cpu, control);
        }
        Load16(ld16) => {
            load16(cpu, ld16);
        }
        Push(op) => {
            push(cpu, op);
        }
        Pop(op) => {
            pop(cpu, op);
        }
        Load8(to, from) => {
            load8(cpu, to, from);
        }
        Arithmetic16(op) => {
            arithmetic16(cpu, op);
        }
        Arithmetic8(op) => {
            arithmetic8(cpu, op);
        }
        JumpRelative(jr) => {
            jump_relative(cpu, jr);
        }
        Jump(jp) => {
            jump(cpu, jp);
        }
        Restart(arg) => {
            restart(cpu, arg);
        }
        Return(op) => {
            ret(cpu, op);
        }
        Call(op) => {
            call(cpu, op);
        }
        BitOp(op) => {
            bit_op(cpu, op);
        }
    }
}

fn cb_instruction(cpu: &mut CPU) {
    cpu.cb_mode = true;
}

pub(crate) fn control_instruction(cpu: &mut CPU, control: ControlOps) {
    match control {
        ControlOps::NOP => nop(cpu),
        ControlOps::STOP => {}
        ControlOps::HALT => {}
        ControlOps::DI => {}
        ControlOps::EI => {}
        ControlOps::DAA => {}
        ControlOps::SCF => {}
        ControlOps::CPL => {}
        ControlOps::CCF => {}
    }

    fn nop(cpu: &mut CPU) {
        cpu.program_counter += 1;
    }
}

pub(crate) fn load16(cpu: &mut CPU, ld16: Ld16) {
    match ld16 {
        Ld16::BCU16 => {}
        Ld16::DEU16 => {}
        Ld16::HLU16 => {}
        Ld16::SPU16 => {}
        Ld16::U16SP => {}
        Ld16::SPHL => {}
    }
}

pub(crate) fn push(cpu: &mut CPU, push: PushPop) {
    match push {
        PushPop::BC => {}
        PushPop::DE => {}
        PushPop::HL => {}
        PushPop::AF => {}
    }
}

pub(crate) fn pop(cpu: &mut CPU, pop: PushPop) {
    match pop {
        PushPop::BC => {}
        PushPop::DE => {}
        PushPop::HL => {}
        PushPop::AF => {}
    }
}

pub(crate) fn load8(cpu: &mut CPU, to: Ld8, from: Ld8) {
    match to {
        Ld8::A => {}
        Ld8::B => {}
        Ld8::C => {}
        Ld8::D => {}
        Ld8::E => {}
        Ld8::H => {}
        Ld8::L => {}
        Ld8::HL => {}
        Ld8::HLPlus => {}
        Ld8::HLMinus => {}
        Ld8::BC => {}
        Ld8::DE => {}
        Ld8::U16 => {}
        Ld8::U8 => {}
        Ld8::FF00AddU8 => {}
        Ld8::FF00AddC => {}
    }

    fn get_from_value(cpu: &CPU, from: Ld8) {
        match from {
            Ld8::A => {}
            Ld8::B => {}
            Ld8::C => {}
            Ld8::D => {}
            Ld8::E => {}
            Ld8::H => {}
            Ld8::L => {}
            Ld8::HL => {}
            Ld8::HLPlus => {}
            Ld8::HLMinus => {}
            Ld8::BC => {}
            Ld8::DE => {}
            Ld8::U16 => {}
            Ld8::U8 => {}
            Ld8::FF00AddU8 => {}
            Ld8::FF00AddC => {}
        }
    }
}

pub(crate) fn arithmetic16(cpu: &mut CPU, op: A16Ops) {
    match op {
        A16Ops::Inc(arg) => {}
        A16Ops::Dec(arg) => {}
        A16Ops::Add(arg) => {}
        A16Ops::AddI8 => {}
        A16Ops::LdI8 => {}
    }
}

pub(crate) fn arithmetic8(cpu: &mut CPU, op: A8Ops) {
    match op {
        A8Ops::Inc(arg) => {}
        A8Ops::Dec(arg) => {}
        A8Ops::Add(arg) => {}
        A8Ops::AddCarry(arg) => {}
        A8Ops::Sub(arg) => {}
        A8Ops::SubCarry(arg) => {}
        A8Ops::And(arg) => {}
        A8Ops::Or(arg) => {}
        A8Ops::Xor(arg) => {}
        A8Ops::Cmp(arg) => {}
    }
}

pub(crate) fn jump_relative(cpu: &mut CPU, jr: JR) {
    match jr {
        JR::I8 => {}
        JR::NC => {}
        JR::NZ => {}
        JR::Z => {}
        JR::C => {}
    }
}

pub(crate) fn jump(cpu: &mut CPU, jp: JP) {
    match jp {
        JP::U16 => {}
        JP::HL => {}
        JP::NZ => {}
        JP::NC => {}
        JP::Z => {}
        JP::C => {}
    }
}

pub(crate) fn restart(cpu: &mut CPU, arg: u8) {
    match arg {
        0 => {}
        1 => {}
        2 => {}
        3 => {}
        4 => {}
        5 => {}
        6 => {}
        7 => {}
        _ => {}
    }
}

pub(crate) fn ret(cpu: &mut CPU, op: Ret) {
    match op {
        Ret::NZ => {}
        Ret::NC => {}
        Ret::Z => {}
        Ret::C => {}
        Ret::None => {}
        Ret::I => {}
    }
}

pub(crate) fn call(cpu: &mut CPU, op: Calls) {
    match op {
        Calls::NZ => {}
        Calls::NC => {}
        Calls::Z => {}
        Calls::C => {}
        Calls::U16 => {}
    }
}

pub(crate) fn bit_op(cpu: &mut CPU, op: BitOps) {
    match op {
        BitOps::RLCA => {}
        BitOps::RLA => {}
        BitOps::RRCA => {}
        BitOps::RRA => {}
        BitOps::RLC(arg) => {}
        BitOps::RRC(arg) => {}
        BitOps::RL(arg) => {}
        BitOps::RR(arg) => {}
        BitOps::SLA(arg) => {}
        BitOps::SRA(arg) => {}
        BitOps::Swap(arg) => {}
        BitOps::SRL(arg) => {}
        BitOps::Bit(value, arg) => {}
        BitOps::Reset(value, arg) => {}
        BitOps::Set(value, arg) => {}
    }
}
