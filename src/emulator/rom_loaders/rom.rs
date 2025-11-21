use std::fs;

pub struct Rom {
    pub bytes: Vec<u8>,
}

impl Rom {
    /// Initalize an empty Rom object.
    pub fn new() -> Rom {
        Rom { bytes: vec![] }
    }

    /// Try to initalize a Rom object with a rom file loaded from path.
    pub fn try_new(path: &str) -> Result<Rom, String> {
        let rom_bytes = fs::read(path);
        match rom_bytes {
            Ok(value) => Ok(Rom { bytes: value }),
            Err(_) => Err("INVALID ROM PATH".to_string()),
        }
    }
}
