use crate::emulator::registers::{Flags, Register16, Register8, Registers};

pub(crate) struct Cpu {
    registers: Registers,
    pub(crate) enable_interrupts: bool,
}

impl Cpu {
    pub(crate) fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            enable_interrupts: false,
        }
    }

    pub(crate) fn get_register(&self, register: &Register8) -> u8 {
        match register {
            Register8::A => self.registers.a,
            Register8::B => self.registers.b,
            Register8::C => self.registers.c,
            Register8::D => self.registers.d,
            Register8::E => self.registers.e,
            Register8::F => self.registers.f,
            Register8::H => self.registers.h,
            Register8::L => self.registers.l,
            Register8::SpLow => self.registers.sp as u8,
            Register8::SpHigh => (self.registers.sp >> 8) as u8,
            Register8::PcLow => self.registers.pc as u8,
            Register8::PcHigh => (self.registers.pc >> 8) as u8,
            Register8::X => self.registers.x,
            Register8::Y => self.registers.y,
        }
    }

    pub(crate) fn set_register(&mut self, value: u8, register: &Register8) {
        match register {
            Register8::A => self.registers.a = value,
            Register8::B => self.registers.b = value,
            Register8::C => self.registers.c = value,
            Register8::D => self.registers.d = value,
            Register8::E => self.registers.e = value,
            Register8::F => self.registers.f = value,
            Register8::H => self.registers.h = value,
            Register8::L => self.registers.l = value,
            Register8::SpLow => {
                self.registers.sp = (self.registers.sp & 0xFF00) + (value as u16);
            }
            Register8::SpHigh => {
                self.registers.sp = (self.registers.sp & 0x00FF) + ((value as u16) << 8);
            }
            Register8::PcLow => {
                self.registers.pc = (self.registers.pc & 0xFF00) + (value as u16);
            }
            Register8::PcHigh => {
                self.registers.pc = (self.registers.pc & 0x00FF) + ((value as u16) << 8);
            }
            Register8::X => self.registers.x = value,
            Register8::Y => self.registers.y = value,
        }
    }

    pub(crate) fn get_register_16(&self, register: &Register16) -> u16 {
        match register {
            Register16::Af => ((self.registers.a as u16) << 8) + (self.registers.f as u16),
            Register16::Bc => ((self.registers.b as u16) << 8) + (self.registers.c as u16),
            Register16::De => ((self.registers.d as u16) << 8) + (self.registers.e as u16),
            Register16::Hl => ((self.registers.h as u16) << 8) + (self.registers.l as u16),
            Register16::Sp => self.registers.sp,
            Register16::Pc => self.registers.pc,
            Register16::Bus => self.registers.bus,
            Register16::Xy => ((self.registers.x as u16) << 8) + (self.registers.y as u16),
        }
    }

    pub(crate) fn set_register_16(&mut self, value: u16, register: &Register16) {
        match register {
            Register16::Af => {
                self.registers.a = (value >> 8) as u8;
                self.registers.f = value as u8;
            }
            Register16::Bc => {
                self.registers.b = (value >> 8) as u8;
                self.registers.c = value as u8;
            }
            Register16::De => {
                self.registers.d = (value >> 8) as u8;
                self.registers.e = value as u8;
            }
            Register16::Hl => {
                self.registers.h = (value >> 8) as u8;
                self.registers.l = value as u8;
            }
            Register16::Sp => {
                self.registers.sp = value;
            }
            Register16::Pc => {
                self.registers.pc = value;
            }
            Register16::Bus => {
                self.registers.bus = value;
            }
            Register16::Xy => {
                self.registers.x = (value >> 8) as u8;
                self.registers.y = value as u8;
            }
        }
    }

    pub(crate) fn get_flag(&mut self, flag: &Flags) -> bool {
        match flag {
            Flags::Z => ((self.registers.f >> 7) & 1) == 1,
            Flags::N => ((self.registers.f >> 6) & 1) == 1,
            Flags::H => ((self.registers.f >> 5) & 1) == 1,
            Flags::C => ((self.registers.f >> 4) & 1) == 1,
        }
    }

    pub(crate) fn set_flag(&mut self, value: bool, flag: &Flags) {
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
