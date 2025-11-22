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
    pub fn new() -> Ram {
        Self::default()
    }

    pub fn fetch(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn fetch_signed(&self, address: u16) -> i8 {
        self.memory[address as usize] as i8
    }

    pub fn fetch_16(&self, address: u16) -> u16 {
        let address = address as usize;

        ((self.memory[address + 1] as u16) << 8) + (self.memory[address] as u16)
    }

    pub fn set(&mut self, value: u8, address: u16) {
        self.memory[address as usize] = value;
    }

    pub fn set_16(&mut self, value: u16, address: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0xFF) as u8;

        let address = address as usize;
        self.memory[address] = high_byte;
        self.memory[address + 1] = low_byte;
    }
}
