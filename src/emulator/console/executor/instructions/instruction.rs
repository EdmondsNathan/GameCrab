pub(crate) enum Instruction {
    Cb,
    Control(ControlOps),
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

pub(crate) enum ControlOps {
    Nop,  //0x00
    Stop, //0x10
    Halt, //0x76
    Di,   //0xF3
    Ei,   //0xFB
    Daa,  //0x27
    Scf,  //0x37
    Cpl,  //0x2F
    Ccf,  //0x3F
}

pub enum JR {
    I8, //18, 2-12
    Nc, //20, 2-8/12
    Nz, //30, 2-8/12
    Z,  //28, 2-8/12
    C,  //38, 2-8/12
}

pub enum JP {
    U16, //C2, 3-16
    HL,  //C2, 1-4
    Nz,  //C2, 3-12/16
    Nc,  //C2, 3-12/16
    Z,   //C2, 3-12/16
    C,   //C2, 3-12/16
}

pub enum Ret {
    Nz,   //C0, 1-8/20
    Nc,   //D0, 1-8/20
    Z,    //C8, 1-8/20
    C,    //D8, 1-8/20
    None, //C9, 1-16
    I,    //D9, 1-16
}

pub enum Calls {
    Nz,  //C4, 3-12/24
    Nc,  //D4, 3-12/24
    Z,   //CC, 3-12/24
    C,   //DC, 3-12/24
    U16, //CD, 3-24
}

pub enum Ld16 {
    BcU16, //01, 3-12
    DeU16, //11, 3-12
    HlU16, //21, 3-12
    SpU16, //31, 3-12
    U16Sp, //08, 3-20
    SpHl,  //F9, 1-8
}

pub enum PushPop {
    Bc,
    De,
    Hl,
    Af,
}

pub enum A16Ops {
    Inc(A16Args),
    Dec(A16Args),
    Add(A16Args),
    AddI8,
    LdI8,
}

pub enum A16Args {
    Bc,
    De,
    Hl,
    Sp,
}

pub enum Ld8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    Hl,
    HlPlus,
    HlMinus,
    Bc,
    De,
    U16,
    U8,
    Ff00AddU8,
    Ff00AddC,
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
    Rlca,
    Rla,
    Rrca,
    Rra,
    Rlc(BitArgs),
    Rrc(BitArgs),
    Rl(BitArgs),
    Rr(BitArgs),
    Sla(BitArgs),
    Sra(BitArgs),
    Swap(BitArgs),
    Srl(BitArgs),
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
