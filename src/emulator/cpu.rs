use crate::emulator::registers::{Flags, Register, Registers};

pub(crate) struct CPU {
    pub(crate) registers: Registers,
    pub(crate) enable_interrupts: bool,
}

impl CPU {
    pub(crate) fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            enable_interrupts: false,
        }
    }

    pub(crate) fn get_register(&self, register: Register) -> u8 {
        match register {
            Register::A => self.registers.a,
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::F => self.registers.f,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
            Register::SpLow => self.registers.sp as u8,
            Register::SpHigh => (self.registers.sp >> 8) as u8,
            Register::PcLow => self.registers.pc as u8,
            Register::PcHigh => (self.registers.pc >> 8) as u8,
        }
    }

    pub(crate) fn set_register(&mut self, value: u8, register: Register) {
        match register {
            Register::A => self.registers.a = value,
            Register::B => self.registers.b = value,
            Register::C => self.registers.c = value,
            Register::D => self.registers.d = value,
            Register::E => self.registers.e = value,
            Register::F => self.registers.f = value,
            Register::H => self.registers.h = value,
            Register::L => self.registers.l = value,
            Register::SpLow => {
                self.registers.sp = (self.registers.sp & 0xFF00) + (value as u16);
            }
            Register::SpHigh => {
                self.registers.sp = (self.registers.sp & 0x00FF) + ((value as u16) << 8);
            }
            Register::PcLow => {
                self.registers.pc = (self.registers.pc & 0xFF00) + (value as u16);
            }
            Register::PcHigh => {
                self.registers.pc = (self.registers.pc & 0x00FF) + ((value as u16) << 8);
            }
        }
    }

    pub(crate) fn get_flag(&mut self, flag: Flags) -> bool {
        match flag {
            Flags::Z => ((self.registers.f >> 7) & 1) == 1,
            Flags::N => ((self.registers.f >> 6) & 1) == 1,
            Flags::H => ((self.registers.f >> 5) & 1) == 1,
            Flags::C => ((self.registers.f >> 4) & 1) == 1,
        }
    }

    pub(crate) fn set_flag(&mut self, value: bool, flag: Flags) {
        match flag {
            Flags::Z => {
                let z = (value as u8) << 7;
                self.registers.f = (self.registers.f & 0b01111111) + z;
            }
            Flags::N => {
                let n = (value as u8) << 6;
                self.registers.f = (self.registers.f & 0b10111111) + n;
            }
            Flags::H => {
                let h = (value as u8) << 5;
                self.registers.f = (self.registers.f & 0b11011111) + h;
            }
            Flags::C => {
                let c = (value as u8) << 4;
                self.registers.f = (self.registers.f & 0b11101111) + c;
            }
        }
    }
}
