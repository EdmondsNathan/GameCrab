use crate::emulator::registers::{Register16, Register8};

pub(super) enum Instruction {
    Cb,
    Lsm8(Lsm8),
    Lsm16(Lsm16),
    Alu8(Alu8),
    Alu16(Alu16),
    Rsb(Rsb),
    Branch,
    Misc,
}

pub(super) enum Misc {
    Nop,  // 00
    Stop, // 10
    Halt, // 76
    Di,   // F3
    Ei,   // FB
}

pub(super) enum Branch {
    Jr,
    Ret,
    Jp,
    Call,
    Rst,
}

pub(super) enum Lsm8 {
    Register(Register8),
    U8,
    Bc,
    De,
    Hl,
    HlPlus,
    HlMinus,
    U16,
    AddC,
    AddU8,
}

pub(super) enum Lsm16 {
    Ld(Lsm16From, Lsm16To),
    Push(Register16),
    Pop(Register16),
}

pub(super) enum Lsm16From {
    Register(Register16),
    U16(u16),
}

pub(super) enum Lsm16To {
    Register(Register16),
    U16(u16),
}

pub(super) enum Alu8 {
    Inc,
    Dec,
    Add,
    Adc,
    Sub,
    Sbc,
    And,
    Xor,
    Or,
    Cp,
}

pub(super) enum Alu16 {
    Inc,
    Dec,
    Add,
}

pub(super) enum Rsb {
    Rlca,
    Rrca,
    Rla,
    Rra,
    Rlc(RsbArg),
    Rrc(RsbArg),
    Rl(RsbArg),
    Rr(RsbArg),
    Sla(RsbArg),
    Sra(RsbArg),
    Swap(RsbArg),
    Srl(RsbArg),
    Bit(RsbArg),
    Res(RsbArg),
    Set(RsbArg),
}

pub(super) enum RsbArg {
    Register(Register8),
    Hl,
}
