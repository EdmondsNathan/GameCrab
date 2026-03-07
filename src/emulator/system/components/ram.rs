pub struct Ram {
    memory: [u8; 0x10000],
    div: u16,
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
            memory: [0; 0x10000],
            div: 0,
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
        // GAMEBOY DOCTOR
        // if address == 0xFF44 {
        //     return 0x90;
        // }

        if address == 0xFF04 {
            return (self.div >> 8) as u8;
        }

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

        // Serial port
        if address == 0xFF02 && value == 0x81 {
            let character = self.fetch(0xFF01) as char;
            // print!("{character}");
        }
        //Div register gets reset if any value is written to it
        else if address == 0xFF04 {
            self.div = 0;
            self.memory[address as usize] = 0;
        }
        // OAM DMA transfer
        else if address == 0xFF46 {
            let source_base = (value as u16) << 8;
            for i in 0..160u16 {
                self.memory[0xFE00 + i as usize] = self.memory[source_base as usize + i as usize];
            }
        }
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
        let value_u8 = value as u8;

        let shift = match interrupt {
            Interrupts::VBlank => 0,
            Interrupts::Lcd => 1,
            Interrupts::Timer => 2,
            Interrupts::Serial => 3,
            Interrupts::Joypad => 4,
        };

        // self.set(byte | (value_u8 << shift), 0xFFFF);
        if value {
            self.set(byte | (1 << shift), 0xFFFF);
        } else {
            self.set(byte & !(1 << shift), 0xFFFF);
        }
    }

    pub fn set_if(&mut self, value: bool, interrupt: Interrupts) {
        let byte = self.fetch(0xFF0F);
        let value_u8 = value as u8;

        let shift = match interrupt {
            Interrupts::VBlank => 0,
            Interrupts::Lcd => 1,
            Interrupts::Timer => 2,
            Interrupts::Serial => 3,
            Interrupts::Joypad => 4,
        };

        // self.set(byte | (value << shift), 0xFF0F);
        if value {
            self.set(byte | (1 << shift), 0xFF0F);
        } else {
            self.set(byte & !(1 << shift), 0xFF0F);
        }
    }

    pub fn get_div(&mut self) -> u16 {
        self.div
    }

    pub fn set_div(&mut self, value: u16) {
        self.div = value;
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
