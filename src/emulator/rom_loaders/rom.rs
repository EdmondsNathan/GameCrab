use std::fs;

pub struct ROM {
    pub bytes: Vec<u8>,
}

impl ROM {
    pub fn new() -> ROM {
        ROM { bytes: vec![] }
    }

    pub fn try_new(path: &str) -> Result<ROM, String> {
        let rom_bytes = fs::read(path);
        match rom_bytes {
            Ok(value) => { Ok(ROM { bytes: value, }) }
            Err(_) => { Err("INVALID ROM PATH".to_string()) }
        }
    }
}