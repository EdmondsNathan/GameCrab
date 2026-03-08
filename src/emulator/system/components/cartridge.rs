pub trait Mbc {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct Cartridge {
    mbc: Box<dyn Mbc>,
}

impl Default for Cartridge {
    fn default() -> Self {
        Cartridge {
            mbc: Box::new(NoMbc {
                rom: vec![0; 0x8000],
                ram: vec![0; 0x2000],
            }),
        }
    }
}

impl Cartridge {
    pub fn from_rom(bytes: Vec<u8>) -> Cartridge {
        let mbc_type = if bytes.len() > 0x0147 {
            bytes[0x0147]
        } else {
            0x00
        };

        let rom_size = if bytes.len() > 0x0148 {
            0x8000 << bytes[0x0148]
        } else {
            bytes.len()
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

        let mbc: Box<dyn Mbc> = match mbc_type {
            0x00 => Box::new(NoMbc {
                rom: bytes,
                ram: vec![0; ram_size],
            }),
            0x01..=0x03 => Box::new(Mbc1::new(bytes, ram_size)),
            0x0F..=0x13 => Box::new(Mbc3::new(bytes, ram_size)),
            _ => {
                eprintln!(
                    "Warning: unsupported MBC type 0x{:02X}, falling back to NoMbc",
                    mbc_type
                );
                Box::new(NoMbc {
                    rom: bytes,
                    ram: vec![0; ram_size],
                })
            }
        };

        Cartridge { mbc }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mbc.read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.mbc.write(address, value);
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
    fn new(rom: Vec<u8>, ram_size: usize) -> Self {
        Mbc1 {
            rom,
            ram: vec![0; ram_size],
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
}

struct Mbc3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
}

impl Mbc3 {
    fn new(rom: Vec<u8>, ram_size: usize) -> Self {
        Mbc3 {
            rom,
            ram: vec![0; ram_size],
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
}
