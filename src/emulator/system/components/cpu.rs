use crate::emulator::system::components::registers::{Flags, Register16, Register8, Registers};

#[derive(Default)]
pub(crate) struct Cpu {
    registers: Registers,
    ime_pending: bool,
    ime: bool,
    is_stopped: bool,
    is_halted: bool,
    halt_bug: bool,
}

impl Cpu {
    /// Create a new cpu object.
    pub(crate) fn new() -> Cpu {
        Self::default()
    }

    /// Get the value of a register.
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
            Register8::BusLow => self.registers.bus as u8,
            Register8::BusHigh => (self.registers.bus >> 8) as u8,
        }
    }

    /// Set the value of a register.
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
            Register8::BusLow => {
                self.registers.bus = (self.registers.bus & 0xFF00) + (value as u16);
            }
            Register8::BusHigh => {
                self.registers.bus = (self.registers.bus & 0x00FF) + ((value as u16) << 8);
            }
        }
    }

    /// Get the value of a register pair.
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

    /// Set value of a register pair.
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

    /// Get the value of a flag.
    pub(crate) fn get_flag(&self, flag: &Flags) -> bool {
        match flag {
            Flags::Z => ((self.registers.f >> 7) & 1) == 1,
            Flags::N => ((self.registers.f >> 6) & 1) == 1,
            Flags::H => ((self.registers.f >> 5) & 1) == 1,
            Flags::C => ((self.registers.f >> 4) & 1) == 1,
        }
    }

    /// Set the value of a flag.
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

    pub(crate) fn get_ime(&self) -> bool {
        self.ime
    }

    pub(crate) fn set_ime(&mut self, value: bool) {
        self.ime = value;
    }

    pub(crate) fn get_is_stopped(&self) -> bool {
        self.is_stopped
    }

    pub(crate) fn set_is_stopped(&mut self, value: bool) {
        self.is_stopped = value;
    }

    pub(crate) fn get_halt(&self) -> bool {
        self.is_halted
    }

    pub(crate) fn set_halt(&mut self, state: bool) {
        self.is_halted = state;
    }

    pub(crate) fn get_ime_pending(&self) -> bool {
        self.ime_pending
    }

    pub(crate) fn set_ime_pending(&mut self, value: bool) {
        self.ime_pending = value;
    }

    pub(crate) fn get_halt_bug(&self) -> bool {
        self.halt_bug
    }

    pub(crate) fn set_halt_bug(&mut self, value: bool) {
        self.halt_bug = value;
    }
}
