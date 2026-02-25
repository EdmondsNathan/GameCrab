use crate::emulator::system::{
    components::registers::{Register16, Register8},
    console::Console,
};

pub(crate) struct Instruction {
    micro_ops: Vec<MicroOp>,
}

pub(crate) struct MicroOp {
    offset: u8,
    micro_op: Operations,
}

pub(crate) enum Operations {
    FetchOpcode,
    Read(Register8),
    Write(Register8),
    Address(Register16),
    //Destination, Source
    SetRegister(Register8, Register8),
    Increment8(Register8),
    Increment16(Register16),
    Arbitrary(fn(&mut Console)),
}
