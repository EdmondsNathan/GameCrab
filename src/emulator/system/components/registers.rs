pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub bus: u16,
    pub x: u8,
    pub y: u8,
}

pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    SpLow,
    SpHigh,
    PcLow,
    PcHigh,
    X,
    Y,
    BusLow,
    BusHigh,
}

pub enum Register16 {
    Af,
    Bc,
    De,
    Hl,
    Sp,
    Pc,
    Bus,
    Xy,
}

pub enum Flags {
    Z,
    N,
    H,
    C,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0x01,
            // b: 0xFF,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            // e: 0xC1,
            e: 0xD8,
            // f: 0x00,
            f: 0xB0,
            // h: 0x84,
            h: 0x01,
            // l: 0x03,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
            x: 0x0,
            y: 0x0,
            bus: 0x0100,
        }
    }
}

impl Registers {
    pub fn new() -> Registers {
        Self::default()
    }
}

impl Register16 {
    /// Splits a register16 into the two register8 (High, Low)
    pub fn register16_to_register8(&self) -> (Register8, Register8) {
        match self {
            Register16::Af => (Register8::A, Register8::F),
            Register16::Bc => (Register8::B, Register8::C),
            Register16::De => (Register8::D, Register8::E),
            Register16::Hl => (Register8::H, Register8::L),
            Register16::Sp => (Register8::SpHigh, Register8::SpLow),
            Register16::Pc => (Register8::PcHigh, Register8::PcLow),
            Register16::Bus => (Register8::BusHigh, Register8::BusLow),
            Register16::Xy => (Register8::X, Register8::Y),
        }
    }
}
