use crate::emulator::instruction::Instruction::*;
use crate::emulator::instruction::Control::*;
use crate::emulator::instruction::Ld16::*;
use crate::emulator::instruction::BitOps::*;
use crate::emulator::instruction::*;

pub fn decode(byte: u8) -> Result<Instruction, u8> {
    let high_nibble = byte & 0xF0;
    let low_nibble = byte & 0x0F;

    match high_nibble {
        0x0 => {
            match low_nibble {
                0x0 => { Ok(Control(NOP)) }
                0x1 => { Ok(Load16(BCU16)) }
                0x2 => { Ok(Load8(Ld8::BC, Ld8::A)) }
                0x3 => { Ok(Arithmetic16(A16Ops::Inc(A16Args::BC))) }
                0x4 => { Ok(Arithmetic8(A8Ops::Inc(A8Args::B))) }
                0x5 => { Ok(Arithmetic8(A8Ops::Dec(A8Args::B))) }
                0x6 => { Ok(Load8(Ld8::B, Ld8::U8)) }
                0x7 => { Ok(BitOp(RLCA)) }
                0x8 => { Ok(Load16(U16SP)) }
                0x9 => { Ok(Arithmetic16(A16Ops::Add(A16Args::BC))) }
                0xA => { Ok(Load8(Ld8::A, Ld8::BC)) }
                0xB => { Ok(Arithmetic16(A16Ops::Dec(A16Args::BC))) }
                0xC => { Ok(Arithmetic8(A8Ops::Inc(A8Args::C))) }
                0xD => { Ok(Arithmetic8(A8Ops::Dec(A8Args::C))) }
                0xE => { Ok(Load8(Ld8::C, Ld8::U8)) }
                0xF => { Ok(BitOp(RRCA)) }
                _ =>   { Err(byte) }
            }
        }
        0x1 => {
            match low_nibble {
                0x0 => { Ok(Control(STOP)) }
                0x1 => { Ok(Load16(DEU16)) }
                0x2 => { Ok(Load8(Ld8::DE, Ld8::A)) }
                0x3 => { Ok(Arithmetic16(A16Ops::Inc(A16Args::DE))) }
                0x4 => { Ok(Arithmetic8(A8Ops::Inc(A8Args::D))) }
                0x5 => { Ok(Arithmetic8(A8Ops::Dec(A8Args::D))) }
                0x6 => { Ok(Load8(Ld8::D, Ld8::U8)) }
                0x7 => { Ok(BitOp(RLA)) }
                0x8 => { Ok(JumpRelative(JR::I8)) }
                0x9 => { Ok(Arithmetic16(A16Ops::Add(A16Args::DE))) }
                0xA => { Ok(Load8(Ld8::A, Ld8::DE)) }
                0xB => { Ok(Arithmetic16(A16Ops::Dec(A16Args::DE))) }
                0xC => { Ok(Arithmetic8(A8Ops::Inc(A8Args::E))) }
                0xD => { Ok(Arithmetic8(A8Ops::Dec(A8Args::E))) }
                0xE => { Ok(Load8(Ld8::E, Ld8::U8)) }
                0xF => { Ok(BitOp(RRA)) }
                _ =>   { Err(byte) }
            }
        }
        0x2 => {
            match low_nibble {
                0x0 => { Ok(JumpRelative(JR::NZ)) }
                0x1 => { Ok(Load16(HLU16)) }
                0x2 => { Ok(Load8(Ld8::HLPlus, Ld8::A)) }
                0x3 => { Ok(Arithmetic16(A16Ops::Inc(A16Args::HL))) }
                0x4 => { Ok(Arithmetic8(A8Ops::Inc(A8Args::H))) }
                0x5 => { Ok(Arithmetic8(A8Ops::Dec(A8Args::H))) }
                0x6 => { Ok(Load8(Ld8::H, Ld8::U8)) }
                0x7 => { Ok(Control(DAA)) }
                0x8 => { Ok(JumpRelative(JR::Z)) }
                0x9 => { Ok(Arithmetic16(A16Ops::Add(A16Args::HL))) }
                0xA => { Ok(Load8(Ld8::A, Ld8::HLPlus)) }
                0xB => { Ok(Arithmetic16(A16Ops::Dec(A16Args::HL))) }
                0xC => { Ok(Arithmetic8(A8Ops::Inc(A8Args::L))) }
                0xD => { Ok(Arithmetic8(A8Ops::Dec(A8Args::L))) }
                0xE => { Ok(Load8(Ld8::A, Ld8::U8)) }
                0xF => { Ok(Control(CPL)) }
                _ =>   { Err(byte) }
            }
        }
        0x3 => {
            match low_nibble {
                0x0 => { Ok(JumpRelative(JR::NC)) }
                0x1 => { Ok(Load16(Ld16::SPU16)) }
                0x2 => { Ok(Load8(Ld8::HLMinus, Ld8::A)) }
                0x3 => { Ok(Arithmetic16(A16Ops::Inc(A16Args::SP))) }
                0x4 => { Ok(Arithmetic8(A8Ops::Inc(A8Args::HL))) }
                0x5 => { Ok(Arithmetic8(A8Ops::Dec(A8Args::HL))) }
                0x6 => { Ok(Load8(Ld8::HL, Ld8::U8)) }
                0x7 => { Ok(Control(SCF)) }
                0x8 => { Ok(JumpRelative(JR::C)) }
                0x9 => { Ok(Arithmetic16(A16Ops::Add(A16Args::SP))) }
                0xA => { Ok(Load8(Ld8::A, Ld8::HLMinus)) }
                0xB => { Ok(Arithmetic16(A16Ops::Dec(A16Args::SP))) }
                0xC => { Ok(Arithmetic8(A8Ops::Inc(A8Args::A))) }
                0xD => { Ok(Arithmetic8(A8Ops::Dec(A8Args::A))) }
                0xE => { Ok(Load8(Ld8::A, Ld8::U8)) }
                0xF => { Ok(Control(CCF)) }
                _ =>   { Err(byte) }
            }
        }
        0x4 => {
            match low_nibble {
                0x0 => { Ok(Load8(Ld8::B, Ld8::B)) }
                0x1 => { Ok(Load8(Ld8::B, Ld8::C)) }
                0x2 => { Ok(Load8(Ld8::B, Ld8::D)) }
                0x3 => { Ok(Load8(Ld8::B, Ld8::E)) }
                0x4 => { Ok(Load8(Ld8::B, Ld8::H)) }
                0x5 => { Ok(Load8(Ld8::B, Ld8::L)) }
                0x6 => { Ok(Load8(Ld8::B, Ld8::HL)) }
                0x7 => { Ok(Load8(Ld8::B, Ld8::A)) }
                0x8 => { Ok(Load8(Ld8::C, Ld8::B)) }
                0x9 => { Ok(Load8(Ld8::C, Ld8::C)) }
                0xA => { Ok(Load8(Ld8::C, Ld8::D)) }
                0xB => { Ok(Load8(Ld8::C, Ld8::E)) }
                0xC => { Ok(Load8(Ld8::C, Ld8::H)) }
                0xD => { Ok(Load8(Ld8::C, Ld8::L)) }
                0xE => { Ok(Load8(Ld8::C, Ld8::HL)) }
                0xF => { Ok(Load8(Ld8::C, Ld8::A)) }
                _ =>   { Err(byte) }
            }
        }
        0x5 => {
            match low_nibble {
                0x0 => { Ok(Load8(Ld8::D, Ld8::B)) }
                0x1 => { Ok(Load8(Ld8::D, Ld8::C)) }
                0x2 => { Ok(Load8(Ld8::D, Ld8::D)) }
                0x3 => { Ok(Load8(Ld8::D, Ld8::E)) }
                0x4 => { Ok(Load8(Ld8::D, Ld8::H)) }
                0x5 => { Ok(Load8(Ld8::D, Ld8::L)) }
                0x6 => { Ok(Load8(Ld8::D, Ld8::HL)) }
                0x7 => { Ok(Load8(Ld8::D, Ld8::A)) }
                0x8 => { Ok(Load8(Ld8::E, Ld8::B)) }
                0x9 => { Ok(Load8(Ld8::E, Ld8::C)) }
                0xA => { Ok(Load8(Ld8::E, Ld8::D)) }
                0xB => { Ok(Load8(Ld8::E, Ld8::E)) }
                0xC => { Ok(Load8(Ld8::E, Ld8::H)) }
                0xD => { Ok(Load8(Ld8::E, Ld8::L)) }
                0xE => { Ok(Load8(Ld8::E, Ld8::HL)) }
                0xF => { Ok(Load8(Ld8::E, Ld8::A)) }
                _ =>   { Err(byte) }
            }
        }
        0x6 => {
            match low_nibble {
                0x0 => { Ok(Load8(Ld8::H, Ld8::B)) }
                0x1 => { Ok(Load8(Ld8::H, Ld8::C)) }
                0x2 => { Ok(Load8(Ld8::H, Ld8::D)) }
                0x3 => { Ok(Load8(Ld8::H, Ld8::E)) }
                0x4 => { Ok(Load8(Ld8::H, Ld8::H)) }
                0x5 => { Ok(Load8(Ld8::H, Ld8::L)) }
                0x6 => { Ok(Load8(Ld8::H, Ld8::HL)) }
                0x7 => { Ok(Load8(Ld8::H, Ld8::A)) }
                0x8 => { Ok(Load8(Ld8::L, Ld8::B)) }
                0x9 => { Ok(Load8(Ld8::L, Ld8::C)) }
                0xA => { Ok(Load8(Ld8::L, Ld8::D)) }
                0xB => { Ok(Load8(Ld8::L, Ld8::E)) }
                0xC => { Ok(Load8(Ld8::L, Ld8::H)) }
                0xD => { Ok(Load8(Ld8::L, Ld8::L)) }
                0xE => { Ok(Load8(Ld8::L, Ld8::HL)) }
                0xF => { Ok(Load8(Ld8::L, Ld8::A)) }
                _ =>   { Err(byte) }
            }
        }
        0x7 => {
            match low_nibble {
                0x0 => { Ok(Load8(Ld8::HL, Ld8::B)) }
                0x1 => { Ok(Load8(Ld8::HL, Ld8::C)) }
                0x2 => { Ok(Load8(Ld8::HL, Ld8::D)) }
                0x3 => { Ok(Load8(Ld8::HL, Ld8::E)) }
                0x4 => { Ok(Load8(Ld8::HL, Ld8::H)) }
                0x5 => { Ok(Load8(Ld8::HL, Ld8::L)) }
                0x6 => { Ok(Control(HALT)) }
                0x7 => { Ok(Load8(Ld8::HL, Ld8::A)) }
                0x8 => { Ok(Load8(Ld8::A, Ld8::B)) }
                0x9 => { Ok(Load8(Ld8::A, Ld8::C)) }
                0xA => { Ok(Load8(Ld8::A, Ld8::D)) }
                0xB => { Ok(Load8(Ld8::A, Ld8::E)) }
                0xC => { Ok(Load8(Ld8::A, Ld8::H)) }
                0xD => { Ok(Load8(Ld8::A, Ld8::L)) }
                0xE => { Ok(Load8(Ld8::A, Ld8::HL)) }
                0xF => { Ok(Load8(Ld8::A, Ld8::A)) }
                _ =>   { Err(byte) }
            }
        }
        0x8 => {
            match low_nibble {
                0x0 => { Ok(Arithmetic8(A8Ops::Add(A8Args::B))) }
                0x1 => { Ok(Arithmetic8(A8Ops::Add(A8Args::C))) }
                0x2 => { Ok(Arithmetic8(A8Ops::Add(A8Args::D))) }
                0x3 => { Ok(Arithmetic8(A8Ops::Add(A8Args::E))) }
                0x4 => { Ok(Arithmetic8(A8Ops::Add(A8Args::H))) }
                0x5 => { Ok(Arithmetic8(A8Ops::Add(A8Args::L))) }
                0x6 => { Ok(Arithmetic8(A8Ops::Add(A8Args::HL))) }
                0x7 => { Ok(Arithmetic8(A8Ops::Add(A8Args::A))) }
                0x8 => { Ok(Arithmetic8(A8Ops::AddCarry(A8Args::B))) }
                0x9 => { Ok(Arithmetic8(A8Ops::AddCarry(A8Args::C))) }
                0xA => { Ok(Arithmetic8(A8Ops::AddCarry(A8Args::D))) }
                0xB => { Ok(Arithmetic8(A8Ops::AddCarry(A8Args::E))) }
                0xC => { Ok(Arithmetic8(A8Ops::AddCarry(A8Args::H))) }
                0xD => { Ok(Arithmetic8(A8Ops::AddCarry(A8Args::L))) }
                0xE => { Ok(Arithmetic8(A8Ops::AddCarry(A8Args::HL))) }
                0xF => { Ok(Arithmetic8(A8Ops::AddCarry(A8Args::A))) }
                _ =>   { Err(byte) }
            }
        }
        0x9 => {
            match low_nibble {
                0x0 => { Ok(Arithmetic8(A8Ops::Sub(A8Args::B))) }
                0x1 => { Ok(Arithmetic8(A8Ops::Sub(A8Args::C))) }
                0x2 => { Ok(Arithmetic8(A8Ops::Sub(A8Args::D))) }
                0x3 => { Ok(Arithmetic8(A8Ops::Sub(A8Args::E))) }
                0x4 => { Ok(Arithmetic8(A8Ops::Sub(A8Args::H))) }
                0x5 => { Ok(Arithmetic8(A8Ops::Sub(A8Args::L))) }
                0x6 => { Ok(Arithmetic8(A8Ops::Sub(A8Args::HL))) }
                0x7 => { Ok(Arithmetic8(A8Ops::Sub(A8Args::A))) }
                0x8 => { Ok(Arithmetic8(A8Ops::SubCarry(A8Args::B))) }
                0x9 => { Ok(Arithmetic8(A8Ops::SubCarry(A8Args::C))) }
                0xA => { Ok(Arithmetic8(A8Ops::SubCarry(A8Args::D))) }
                0xB => { Ok(Arithmetic8(A8Ops::SubCarry(A8Args::E))) }
                0xC => { Ok(Arithmetic8(A8Ops::SubCarry(A8Args::H))) }
                0xD => { Ok(Arithmetic8(A8Ops::SubCarry(A8Args::L))) }
                0xE => { Ok(Arithmetic8(A8Ops::SubCarry(A8Args::HL))) }
                0xF => { Ok(Arithmetic8(A8Ops::SubCarry(A8Args::A))) }
                _ =>   { Err(byte) }
            }
        }
        0xA => {
            match low_nibble {
                0x0 => { Ok(Arithmetic8(A8Ops::And(A8Args::B))) }
                0x1 => { Ok(Arithmetic8(A8Ops::And(A8Args::C))) }
                0x2 => { Ok(Arithmetic8(A8Ops::And(A8Args::D))) }
                0x3 => { Ok(Arithmetic8(A8Ops::And(A8Args::E))) }
                0x4 => { Ok(Arithmetic8(A8Ops::And(A8Args::H))) }
                0x5 => { Ok(Arithmetic8(A8Ops::And(A8Args::L))) }
                0x6 => { Ok(Arithmetic8(A8Ops::And(A8Args::HL))) }
                0x7 => { Ok(Arithmetic8(A8Ops::And(A8Args::A))) }
                0x8 => { Ok(Arithmetic8(A8Ops::Xor(A8Args::B))) }
                0x9 => { Ok(Arithmetic8(A8Ops::Xor(A8Args::C))) }
                0xA => { Ok(Arithmetic8(A8Ops::Xor(A8Args::D))) }
                0xB => { Ok(Arithmetic8(A8Ops::Xor(A8Args::E))) }
                0xC => { Ok(Arithmetic8(A8Ops::Xor(A8Args::H))) }
                0xD => { Ok(Arithmetic8(A8Ops::Xor(A8Args::L))) }
                0xE => { Ok(Arithmetic8(A8Ops::Xor(A8Args::HL))) }
                0xF => { Ok(Arithmetic8(A8Ops::Xor(A8Args::A))) }
                _ =>   { Err(byte) }
            }
        }
        0xB => {
            match low_nibble {
                0x0 => { Ok(Arithmetic8(A8Ops::Or(A8Args::B))) }
                0x1 => { Ok(Arithmetic8(A8Ops::Or(A8Args::C))) }
                0x2 => { Ok(Arithmetic8(A8Ops::Or(A8Args::D))) }
                0x3 => { Ok(Arithmetic8(A8Ops::Or(A8Args::E))) }
                0x4 => { Ok(Arithmetic8(A8Ops::Or(A8Args::H))) }
                0x5 => { Ok(Arithmetic8(A8Ops::Or(A8Args::L))) }
                0x6 => { Ok(Arithmetic8(A8Ops::Or(A8Args::HL))) }
                0x7 => { Ok(Arithmetic8(A8Ops::Or(A8Args::A))) }
                0x8 => { Ok(Arithmetic8(A8Ops::Cmp(A8Args::B))) }
                0x9 => { Ok(Arithmetic8(A8Ops::Cmp(A8Args::C))) }
                0xA => { Ok(Arithmetic8(A8Ops::Cmp(A8Args::D))) }
                0xB => { Ok(Arithmetic8(A8Ops::Cmp(A8Args::E))) }
                0xC => { Ok(Arithmetic8(A8Ops::Cmp(A8Args::H))) }
                0xD => { Ok(Arithmetic8(A8Ops::Cmp(A8Args::L))) }
                0xE => { Ok(Arithmetic8(A8Ops::Cmp(A8Args::HL))) }
                0xF => { Ok(Arithmetic8(A8Ops::Cmp(A8Args::A))) }
                _ =>   { Err(byte) }
            }
        }
        0xC => {
            match low_nibble {
                0x0 => { Ok(Return(Ret::NZ)) }
                0x1 => { Ok(Pop(PushPop::BC)) }
                0x2 => { Ok(Jump(JP::NZ)) }
                0x3 => { Ok(Jump(JP::U16)) }
                0x4 => { Ok(Call(Calls::NZ)) }
                0x5 => { Ok(Push(PushPop::BC)) }
                0x6 => { Ok(Arithmetic8(A8Ops::Add(A8Args::U8))) }
                0x7 => { Ok(Restart(0)) }
                0x8 => { Ok(Return(Ret::Z)) }
                0x9 => { Ok(Return(Ret::None)) }
                0xA => { Ok(Jump(JP::Z)) }
                0xB => { Ok(CB) }
                0xC => { Ok(Call(Calls::Z)) }
                0xD => { Ok(Call(Calls::U16)) }
                0xE => { Ok(Arithmetic8(A8Ops::AddCarry(A8Args::U8))) }
                0xF => { Ok(Restart(1)) }
                _ =>   { Err(byte) }
            }
        }
        0xD => {
            match low_nibble {
                0x0 => { Ok(Return(Ret::NC)) }
                0x1 => { Ok(Pop(PushPop::DE)) }
                0x2 => { Ok(Jump(JP::NC)) }
                0x3 => { Err(byte) }
                0x4 => { Ok(Call(Calls::NC)) }
                0x5 => { Ok(Push(PushPop::DE)) }
                0x6 => { Ok(Arithmetic8(A8Ops::Sub(A8Args::U8))) }
                0x7 => { Ok(Restart(2)) }
                0x8 => { Ok(Return(Ret::C)) }
                0x9 => { Ok(Return(Ret::I)) }
                0xA => { Ok(Jump(JP::C)) }
                0xB => { Err(byte) }
                0xC => { Ok(Call(Calls::C)) }
                0xD => { Err(byte) }
                0xE => { Ok(Arithmetic8(A8Ops::SubCarry(A8Args::U8))) }
                0xF => { Ok(Restart(3)) }
                _ =>   { Err(byte) }
            }
        }
        0xE => {
            match low_nibble {
                0x0 => { Ok(Load8(Ld8::FF00AddU8, Ld8::A)) }
                0x1 => { Ok(Pop(PushPop::HL)) }
                0x2 => { Ok(Load8(Ld8::FF00AddC, Ld8::A)) }
                0x3 => { Err(byte) }
                0x4 => { Err(byte) }
                0x5 => { Ok(Push(PushPop::HL)) }
                0x6 => { Ok(Arithmetic8(A8Ops::And(A8Args::U8))) }
                0x7 => { Ok(Restart(4)) }
                0x8 => { Ok(Arithmetic16(A16Ops::AddI8)) }
                0x9 => { Ok(Jump(JP::HL)) }
                0xA => { Ok(Load8(Ld8::U16, Ld8::A)) }
                0xB => { Err(byte) }
                0xC => { Err(byte) }
                0xD => { Err(byte) }
                0xE => { Ok(Arithmetic8(A8Ops::Xor(A8Args::U8))) }
                0xF => { Ok(Restart(5)) }
                _ =>   { Err(byte) }
            }
        }
        0xF => {
            match low_nibble {
                0x0 => { Ok(Load8(Ld8::A, Ld8::FF00AddU8)) }
                0x1 => { Ok(Pop(PushPop::AF)) }
                0x2 => { Ok(Load8(Ld8::A, Ld8::FF00AddC)) }
                0x3 => { Ok(Control(DI)) }
                0x4 => { Err(byte) }
                0x5 => { Ok(Push(PushPop::AF)) }
                0x6 => { Ok(Arithmetic8(A8Ops::Or(A8Args::U8))) }
                0x7 => { Ok(Restart(6)) }
                0x8 => { Ok(Arithmetic16(A16Ops::LdI8)) }
                0x9 => { Ok(Load16(SPHL)) }
                0xA => { Ok(Load8(Ld8::A, Ld8::U16)) }
                0xB => { Ok(Control(EI)) }
                0xC => { Err(byte) }
                0xD => { Err(byte) }
                0xE => { Ok(Arithmetic8(A8Ops::Cmp(A8Args::U8))) }
                0xF => { Ok(Restart(7)) }
                _ =>   { Err(byte) }
            }
        }
        _ => { Err(byte) }
    }
}
