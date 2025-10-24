pub(crate) struct Registers {
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
    pub y: u8,
}

pub(crate) enum Register8 {
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
    Y,
}

pub(crate) enum Register16 {
    Af,
    Bc,
    De,
    Hl,
    Sp,
    Pc,
}

pub(crate) enum Flags {
    Z,
    N,
    H,
    C,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            b: 0xFF,
            c: 0x13,
            d: 0x00,
            e: 0xC1,
            f: 0x00,
            h: 0x84,
            l: 0x03,
            sp: 0xFFFE,
            pc: 0x0100,
            y: 0x0,
        }
    }
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) + self.f as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) + self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) + self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) + self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}
