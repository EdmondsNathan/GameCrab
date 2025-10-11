use crate::emulator::{
    console::Console,
    cpu::CPU,
    registers::{Register16, Register8},
};

pub(crate) enum Command {
    Standard(fn(&mut Console)),
    ReadWrite(fn(&mut Console, u16, Register8), u16, Register8),
    SetRegister(fn(&mut CPU, u8, Register8), u8, Register8),
    SetRegister16(fn(&mut CPU, u16, Register16), u16, Register16),
}
