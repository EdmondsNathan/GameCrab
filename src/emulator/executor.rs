use crate::emulator::cpu::CPU;
use crate::emulator::instruction::Instruction::*;
use crate::emulator::instruction::*;

pub fn cb_instruction() {}

pub fn control_instruction(control: ControlOps) {
    match control {
        ControlOps::NOP => {}
        ControlOps::STOP => {}
        ControlOps::HALT => {}
        ControlOps::DI => {}
        ControlOps::EI => {}
        ControlOps::DAA => {}
        ControlOps::SCF => {}
        ControlOps::CPL => {}
        ControlOps::CCF => {}
    }
}

pub fn load16(ld16: Ld16) {
    match ld16 {
        Ld16::BCU16 => {}
        Ld16::DEU16 => {}
        Ld16::HLU16 => {}
        Ld16::SPU16 => {}
        Ld16::U16SP => {}
        Ld16::SPHL => {}
    }
}

pub fn push(push: PushPop) {
    match push {
        PushPop::BC => {}
        PushPop::DE => {}
        PushPop::HL => {}
        PushPop::AF => {}
    }
}

pub fn pop(pop: PushPop) {
    match pop {
        PushPop::BC => {}
        PushPop::DE => {}
        PushPop::HL => {}
        PushPop::AF => {}
    }
}

pub fn ld8(to: Ld8, from: Ld8) {
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

    fn get_from_value(from: Ld8) {
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

pub fn arithmetic16(op: A16Ops) {
    match op {
        A16Ops::Inc(arg) => {}
        A16Ops::Dec(arg) => {}
        A16Ops::Add(arg) => {}
        A16Ops::AddI8 => {}
        A16Ops::LdI8 => {}
    }
}

pub fn arithmetic8(op: A8Ops) {
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

pub fn jump_relative(jr: JR) {
    match jr {
        JR::I8 => {}
        JR::NC => {}
        JR::NZ => {}
        JR::Z => {}
        JR::C => {}
    }
}

pub fn jump(jp: JP) {
    match jp {
        JP::U16 => {}
        JP::HL => {}
        JP::NZ => {}
        JP::NC => {}
        JP::Z => {}
        JP::C => {}
    }
}

pub fn restart(arg: u8) {
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

pub fn ret(op: Ret) {
    match op {
        Ret::NZ => {}
        Ret::NC => {}
        Ret::Z => {}
        Ret::C => {}
        Ret::None => {}
        Ret::I => {}
    }
}

pub fn call(op: Calls) {
    match op {
        Calls::NZ => {}
        Calls::NC => {}
        Calls::Z => {}
        Calls::C => {}
        Calls::U16 => {}
    }
}

pub fn bit_op(op: BitOps) {
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
