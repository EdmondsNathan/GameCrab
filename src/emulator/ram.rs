pub struct RAM {
    memory: Box<[u8; 0xFFFF]>,
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            memory: Box::new([0; 0xFFFF]),
        }
    }

    pub fn try_fetch(&self, address: u16) -> Result<u8, String> {
        match address {
            u16::MAX => { Err("Tried to access memory address ".to_string() + &address.to_string())}
            _ => { Ok(self.memory[address as usize]) }
        }
    }

    pub fn try_fetch_signed(&self, address: u16) -> Result<i8, String> {
        match self.try_fetch(address) {
            Ok(value) => { Ok(value as i8) }
            Err(error) => { Err(error) }
        }
    }

    pub fn try_fetch_16(&self, address: u16) -> Result<u16, String> {
        let return_value: u16;

        match self.try_fetch(address) {
            Ok(value) => { return_value = (value as u16) << 8; }
            Err(error) => { return Err(error); }
        }

        match self.try_fetch(address + 1) {
            Ok(value) => { Ok(return_value + (value as u16)) }
            Err(error) => { Err(error) }
        }
    }

    pub fn fetch(&self, address: u16) -> u8 {
        match self.try_fetch(address) {
            Ok(value) => { value }
            Err(error) => { self.try_fetch(0).unwrap() }
        }
    }

    pub fn fetch_signed(&self, address: u16) -> i8 {
        match self.try_fetch_signed(address) {
            Ok(value) => { value }
            Err(error) => { self.try_fetch_signed(0).unwrap() }
        }
    }

    pub fn fetch_16(&self, address: u16) -> u16 {
        match self.try_fetch_16(address) {
            Ok(value) => { value }
            Err(error) => { self.try_fetch_16(0).unwrap() }
        }
    }
}