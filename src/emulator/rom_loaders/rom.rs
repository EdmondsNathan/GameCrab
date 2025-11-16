use std::fs;

pub struct Rom {
    pub bytes: Vec<u8>,
}

impl Rom {
    pub fn new() -> Rom {
        Rom { bytes: vec![] }
    }

    pub fn try_new(path: &str) -> Result<Rom, String> {
        let rom_bytes = fs::read(path);
        match rom_bytes {
            Ok(value) => Ok(Rom { bytes: value }),
            Err(_) => Err("INVALID ROM PATH".to_string()),
        }
    }
}
