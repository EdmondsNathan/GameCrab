pub enum Instruction {
    CB,
    Control(Control),
    Load16(Ld16),
    Push(PushPop),
    Pop(PushPop),
    Load8(Ld8, Ld8),
    Arithmetic16(A16Ops),
    Arithmetic8(A8Ops),
    JumpRelative(JR),
    Jump(JP),
    Restart(u8),
    Return(Ret),
    Call(Calls),
    BitOp(BitOps),
}

pub enum Control {
    NOP,  //00, 1-4
    STOP, //10, 1-4
    HALT, //76, 1-4
    DI,   //F3, 1-4
    EI,   //FB, 1-4
    DAA,  //27, 1-4
    SCF,  //37, 1-4
    CPL,  //2F, 1-4
    CCF,  //3F, 1-4
}

pub enum JR {
    I8, //18, 2-12
    NC, //20, 2-8/12
    NZ, //30, 2-8/12
    Z,  //28, 2-8/12
    C,  //38, 2-8/12
}

pub enum JP {
    U16, //C2, 3-16
    HL,  //C2, 1-4
    NZ,  //C2, 3-12/16
    NC,  //C2, 3-12/16
    Z,   //C2, 3-12/16
    C,   //C2, 3-12/16
}

pub enum Ret {
    NZ,   //C0, 1-8/20
    NC,   //D0, 1-8/20
    Z,    //C8, 1-8/20
    C,    //D8, 1-8/20
    None, //C9, 1-16
    I,    //D9, 1-16
}

pub enum Calls {
    NZ,  //C4, 3-12/24
    NC,  //D4, 3-12/24
    Z,   //CC, 3-12/24
    C,   //DC, 3-12/24
    U16, //CD, 3-24
}

pub enum Ld16 {
    BCU16, //01, 3-12
    DEU16, //11, 3-12
    HLU16, //21, 3-12
    SPU16, //31, 3-12
    U16SP, //08, 3-20
    SPHL,  //F9, 1-8
}

pub enum PushPop {
    BC,
    DE,
    HL,
    AF,
}

pub enum A16Ops {
    Inc(A16Args),
    Dec(A16Args),
    Add(A16Args),
    AddI8,
    LdI8,
}

pub enum A16Args {
    BC,
    DE,
    HL,
    SP,
}

pub enum Ld8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    HLPlus,
    HLMinus,
    BC,
    DE,
    U16,
    U8,
    FF00AddU8,
    FF00AddC,
}

pub enum BitArgs {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

pub enum BitOps {
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC(BitArgs),
    RRC(BitArgs),
    RL(BitArgs),
    RR(BitArgs),
    SLA(BitArgs),
    SRA(BitArgs),
    Swap(BitArgs),
    SRL(BitArgs),
    Bit(u8, BitArgs),
    Reset(u8, BitArgs),
    Set(u8, BitArgs),
}

pub enum A8Ops {
    Inc(A8Args),
    Dec(A8Args),
    Add(A8Args),
    AddCarry(A8Args),
    Sub(A8Args),
    SubCarry(A8Args),
    And(A8Args),
    Or(A8Args),
    Xor(A8Args),
    Cmp(A8Args),
}
pub enum A8Args {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    U8,
}
