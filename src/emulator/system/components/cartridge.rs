use std::path::PathBuf;

pub trait Mbc {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn ram(&self) -> &[u8];
}

pub struct Cartridge {
    mbc: Box<dyn Mbc>,
    has_battery: bool,
    save_path: Option<PathBuf>,
    ram_dirty: bool,
}

impl Default for Cartridge {
    fn default() -> Self {
        Cartridge {
            mbc: Box::new(NoMbc {
                rom: vec![0; 0x8000],
                ram: vec![0; 0x2000],
            }),
            has_battery: false,
            save_path: None,
            ram_dirty: false,
        }
    }
}

impl Cartridge {
    pub fn from_rom(bytes: Vec<u8>, rom_path: &str) -> Cartridge {
        let mbc_type = if bytes.len() > 0x0147 {
            bytes[0x0147]
        } else {
            0x00
        };

        let ram_size = if bytes.len() > 0x0149 {
            match bytes[0x0149] {
                0x00 => 0,
                0x01 => 0x800,
                0x02 => 0x2000,
                0x03 => 0x8000,
                0x04 => 0x20000,
                0x05 => 0x10000,
                _ => 0,
            }
        } else {
            0
        };

        let has_battery = matches!(
            mbc_type,
            0x03 | 0x06 | 0x09 | 0x0F | 0x10 | 0x13 | 0x1B | 0x1E
        );

        let save_path = if has_battery {
            let mut path = PathBuf::from(rom_path);
            path.set_extension("sav");
            Some(path)
        } else {
            None
        };

        // Load saved RAM if a .sav file exists
        let saved_ram = if has_battery {
            if let Some(ref path) = save_path {
                std::fs::read(path).ok().map(|mut data| {
                    data.resize(ram_size, 0);
                    data
                })
            } else {
                None
            }
        } else {
            None
        };

        let mbc: Box<dyn Mbc> = match mbc_type {
            0x00 => Box::new(NoMbc {
                rom: bytes,
                ram: saved_ram.unwrap_or_else(|| vec![0; ram_size]),
            }),
            0x01..=0x03 => Box::new(Mbc1::new(bytes, ram_size, saved_ram)),
            0x0F..=0x13 => Box::new(Mbc3::new(bytes, ram_size, saved_ram)),
            _ => {
                eprintln!(
                    "Warning: unsupported MBC type 0x{:02X}, falling back to NoMbc",
                    mbc_type
                );
                Box::new(NoMbc {
                    rom: bytes,
                    ram: saved_ram.unwrap_or_else(|| vec![0; ram_size]),
                })
            }
        };

        Cartridge {
            mbc,
            has_battery,
            save_path,
            ram_dirty: false,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mbc.read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if (0xA000..=0xBFFF).contains(&address) {
            self.ram_dirty = true;
        }
        self.mbc.write(address, value);
    }

    pub fn save_ram(&mut self) {
        if !self.has_battery || !self.ram_dirty {
            return;
        }
        let ram = self.mbc.ram();
        if ram.is_empty() {
            return;
        }
        if let Some(ref path) = self.save_path {
            if let Err(e) = std::fs::write(path, ram) {
                eprintln!("Failed to write save file {:?}: {}", path, e);
            } else {
                self.ram_dirty = false;
            }
        }
    }
}

struct NoMbc {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Mbc for NoMbc {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => {
                let addr = address as usize;
                if addr < self.rom.len() {
                    self.rom[addr]
                } else {
                    0xFF
                }
            }
            0xA000..=0xBFFF => {
                let addr = (address - 0xA000) as usize;
                if addr < self.ram.len() {
                    self.ram[addr]
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => {
                let addr = address as usize;
                if addr < self.rom.len() {
                    self.rom[addr] = value;
                }
            }
            0xA000..=0xBFFF => {
                let addr = (address - 0xA000) as usize;
                if addr < self.ram.len() {
                    self.ram[addr] = value;
                }
            }
            _ => {}
        }
    }

    fn ram(&self) -> &[u8] {
        &self.ram
    }
}

struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    banking_mode: bool,
}

impl Mbc1 {
    fn new(rom: Vec<u8>, ram_size: usize, saved_ram: Option<Vec<u8>>) -> Self {
        Mbc1 {
            rom,
            ram: saved_ram.unwrap_or_else(|| vec![0; ram_size]),
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            banking_mode: false,
        }
    }

    fn rom_bank_number(&self) -> usize {
        let lower = self.rom_bank as usize & 0x1F;
        let upper = (self.ram_bank as usize & 0x03) << 5;
        (upper | lower) % (self.rom.len() / 0x4000).max(1)
    }

    fn bank0_number(&self) -> usize {
        if self.banking_mode {
            let upper = (self.ram_bank as usize & 0x03) << 5;
            upper % (self.rom.len() / 0x4000).max(1)
        } else {
            0
        }
    }

    fn ram_bank_number(&self) -> usize {
        if self.banking_mode {
            (self.ram_bank as usize & 0x03) % (self.ram.len() / 0x2000).max(1)
        } else {
            0
        }
    }
}

impl Mbc for Mbc1 {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                let bank = self.bank0_number();
                let addr = bank * 0x4000 + address as usize;
                if addr < self.rom.len() {
                    self.rom[addr]
                } else {
                    0xFF
                }
            }
            0x4000..=0x7FFF => {
                let bank = self.rom_bank_number();
                let addr = bank * 0x4000 + (address as usize - 0x4000);
                if addr < self.rom.len() {
                    self.rom[addr]
                } else {
                    0xFF
                }
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled || self.ram.is_empty() {
                    return 0xFF;
                }
                let bank = self.ram_bank_number();
                let addr = bank * 0x2000 + (address as usize - 0xA000);
                if addr < self.ram.len() {
                    self.ram[addr]
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            0x2000..=0x3FFF => {
                let mut bank = value & 0x1F;
                if bank == 0 {
                    bank = 1;
                }
                self.rom_bank = bank;
            }
            0x4000..=0x5FFF => {
                self.ram_bank = value & 0x03;
            }
            0x6000..=0x7FFF => {
                self.banking_mode = (value & 0x01) != 0;
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled || self.ram.is_empty() {
                    return;
                }
                let bank = self.ram_bank_number();
                let addr = bank * 0x2000 + (address as usize - 0xA000);
                if addr < self.ram.len() {
                    self.ram[addr] = value;
                }
            }
            _ => {}
        }
    }

    fn ram(&self) -> &[u8] {
        &self.ram
    }
}

struct Mbc3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
}

impl Mbc3 {
    fn new(rom: Vec<u8>, ram_size: usize, saved_ram: Option<Vec<u8>>) -> Self {
        Mbc3 {
            rom,
            ram: saved_ram.unwrap_or_else(|| vec![0; ram_size]),
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
        }
    }

    fn rom_bank_number(&self) -> usize {
        let bank = self.rom_bank as usize & 0x7F;
        if self.rom.is_empty() {
            0
        } else {
            bank % (self.rom.len() / 0x4000).max(1)
        }
    }
}

impl Mbc for Mbc3 {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                let addr = address as usize;
                if addr < self.rom.len() {
                    self.rom[addr]
                } else {
                    0xFF
                }
            }
            0x4000..=0x7FFF => {
                let bank = self.rom_bank_number();
                let addr = bank * 0x4000 + (address as usize - 0x4000);
                if addr < self.rom.len() {
                    self.rom[addr]
                } else {
                    0xFF
                }
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                }
                // RAM bank 0x00-0x03: external RAM
                if self.ram_bank <= 0x03 {
                    let addr = (self.ram_bank as usize) * 0x2000 + (address as usize - 0xA000);
                    if addr < self.ram.len() {
                        self.ram[addr]
                    } else {
                        0xFF
                    }
                } else {
                    // RTC registers (0x08-0x0C) — not implemented, return 0
                    0x00
                }
            }
            _ => 0xFF,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            0x2000..=0x3FFF => {
                let mut bank = value & 0x7F;
                if bank == 0 {
                    bank = 1;
                }
                self.rom_bank = bank;
            }
            0x4000..=0x5FFF => {
                self.ram_bank = value;
            }
            0x6000..=0x7FFF => {
                // RTC latch — not implemented
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return;
                }
                if self.ram_bank <= 0x03 {
                    let addr = (self.ram_bank as usize) * 0x2000 + (address as usize - 0xA000);
                    if addr < self.ram.len() {
                        self.ram[addr] = value;
                    }
                }
                // RTC register writes ignored
            }
            _ => {}
        }
    }

    fn ram(&self) -> &[u8] {
        &self.ram
    }
}
