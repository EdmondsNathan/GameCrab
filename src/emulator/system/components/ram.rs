pub struct Ram {
    memory: Box<[u8; 0x10000]>,
}

pub enum Interrupts {
    VBlank,
    Lcd,
    Timer,
    Serial,
    Joypad,
}

impl Default for Ram {
    fn default() -> Self {
        Ram {
            // memory: Box::new([0; 0xFFFF]),
            memory: Box::new([0; 0x10000]),
        }
    }
}

impl Ram {
    /// Create a new Ram object.
    pub fn new() -> Ram {
        Self::default()
    }

    /// Fetch the value of an address.
    pub fn fetch(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    /// Fetch the signed value of an address.
    pub fn fetch_signed(&self, address: u16) -> i8 {
        self.memory[address as usize] as i8
    }

    /// Fetch two consecutive addresses. The first address is the high byte and the second is the low.
    pub fn fetch_16(&self, address: u16) -> u16 {
        let address = address as usize;

        ((self.memory[address + 1] as u16) << 8) + (self.memory[address] as u16)
    }

    /// Set the value of an address.
    pub fn set(&mut self, value: u8, address: u16) {
        self.memory[address as usize] = value;
    }

    /// Set the value of two consecutive addresses. The high byte is the first address and the low byte is the following.
    pub fn set_16(&mut self, value: u16, address: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0xFF) as u8;

        let address = address as usize;
        self.memory[address] = high_byte;
        self.memory[address + 1] = low_byte;
    }

    pub fn get_ie(&self, interrupt: Interrupts) -> bool {
        let byte = self.fetch(0xFFFF);

        (match interrupt {
            Interrupts::VBlank => byte & 0b00000001,
            Interrupts::Lcd => (byte >> 1) & 0b00000001,
            Interrupts::Timer => (byte >> 2) & 0b00000001,
            Interrupts::Serial => (byte >> 3) & 0b00000001,
            Interrupts::Joypad => (byte >> 4) & 0b00000001,
        } != 0)
    }

    pub fn get_if(&self, interrupt: Interrupts) -> bool {
        let byte = self.fetch(0xFF0F);

        (match interrupt {
            Interrupts::VBlank => byte & 0b00000001,
            Interrupts::Lcd => (byte >> 1) & 0b00000001,
            Interrupts::Timer => (byte >> 2) & 0b00000001,
            Interrupts::Serial => (byte >> 3) & 0b00000001,
            Interrupts::Joypad => (byte >> 4) & 0b00000001,
        } != 0)
    }

    pub fn set_ie(&mut self, value: bool, interrupt: Interrupts) {
        let byte = self.fetch(0xFFFF);
        let value = value as u8;

        let shift = match interrupt {
            Interrupts::VBlank => 0,
            Interrupts::Lcd => 1,
            Interrupts::Timer => 2,
            Interrupts::Serial => 3,
            Interrupts::Joypad => 4,
        };

        self.set(byte | (value << shift), 0xFFFF);
    }

    pub fn set_if(&mut self, value: bool, interrupt: Interrupts) {
        let byte = self.fetch(0xFF0F);
        let value = value as u8;

        let shift = match interrupt {
            Interrupts::VBlank => 0,
            Interrupts::Lcd => 1,
            Interrupts::Timer => 2,
            Interrupts::Serial => 3,
            Interrupts::Joypad => 4,
        };

        self.set(byte | (value << shift), 0xFF0F);
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::system::components::ram::{Interrupts, Ram};

    #[test]
    fn set_ie() {
        let mut ram = Ram::default();

        ram.set_ie(true, Interrupts::VBlank);

        assert!(ram.get_ie(Interrupts::VBlank));
    }
}
