use crate::emulator::registers::{Register, Registers};

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
}
