pub struct Ram {
    memory: Box<[u8; 0xFFFF]>,
}

impl Default for Ram {
    fn default() -> Self {
        Ram {
            memory: Box::new([0; 0xFFFF]),
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
}
