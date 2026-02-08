use crate::emulator::system::components::registers::{Register16, Register8};

pub(crate) struct MicroOp {
    offset: u8,
    micro_op: Operations,
}

pub(crate) enum Operations {
    FetchOpcode,
    Read(Register8),
    Write(u16),
    Address(Register16),
    //Destination, Source
    SetRegister(Register8, Register8),
    Increment8(Register8),
    Increment16(Register16),
}
