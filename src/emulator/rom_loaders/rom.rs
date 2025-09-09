use std::fs;

pub struct Rom {
    bytes: Vec<u8>,
}

impl Rom {
    pub fn try_new(path: &str) -> Result<Rom, String> {
        let rom_bytes = fs::read(path);
        match rom_bytes {
            Ok(value) => { Ok(Rom { bytes: value, }) }
            Err(_) => { Err("INVALID ROM PATH".to_string()) }
        }
    }

    pub fn dump_rom(&self) {
        let mut i = 0;
        for byte in &self.bytes {
            println!("{:X?}: {:X?}", i, byte);
            i += 1;
        }
    }
}