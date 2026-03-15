use std::io::{Read, Write};

use super::cartridge::Cartridge;

// OR masks for reading APU registers (write-only bits read as 1)
const APU_READ_MASKS: [u8; 48] = [
    0x80, // 0xFF10 NR10
    0x3F, // 0xFF11 NR11 (length bits write-only)
    0x00, // 0xFF12 NR12
    0xFF, // 0xFF13 NR13 (write-only)
    0xBF, // 0xFF14 NR14
    0xFF, // 0xFF15 unused
    0x3F, // 0xFF16 NR21
    0x00, // 0xFF17 NR22
    0xFF, // 0xFF18 NR23 (write-only)
    0xBF, // 0xFF19 NR24
    0x7F, // 0xFF1A NR30
    0xFF, // 0xFF1B NR31 (write-only)
    0x9F, // 0xFF1C NR32
    0xFF, // 0xFF1D NR33 (write-only)
    0xBF, // 0xFF1E NR34
    0xFF, // 0xFF1F unused
    0xFF, // 0xFF20 NR41 (write-only)
    0x00, // 0xFF21 NR42
    0x00, // 0xFF22 NR43
    0xBF, // 0xFF23 NR44
    0x00, // 0xFF24 NR50
    0x00, // 0xFF25 NR51
    0x70, // 0xFF26 NR52 (bits 4-6 always set; channel status handled by APU sync)
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 0xFF27-0xFF2F unused
    // 0xFF30-0xFF3F wave RAM (fully readable)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub struct Ram {
    memory: [u8; 0x10000],
    cartridge: Cartridge,
    div: u16,
    /// Action buttons state (active low): bit0=A, bit1=B, bit2=Select, bit3=Start
    pub(crate) joypad_action: u8,
    /// Direction buttons state (active low): bit0=Right, bit1=Left, bit2=Up, bit3=Down
    pub(crate) joypad_direction: u8,
    /// Queued APU register writes for Console to forward to the APU
    pub(crate) apu_register_writes: Vec<(u16, u8)>,
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
            cartridge: Cartridge::default(),
            div: 0,
            joypad_action: 0x0F,
            joypad_direction: 0x0F,
            apu_register_writes: Vec::new(),
        }
    }
}

impl Ram {
    /// Create a new Ram with a cartridge.
    pub fn new(cartridge: Cartridge) -> Ram {
        Ram {
            cartridge,
            ..Default::default()
        }
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

        if address == 0xFF0F {
            return self.memory[0xFF0F] | 0xE0;
        }

        if address == 0xFF00 {
            let select = self.memory[0xFF00] & 0x30;
            let mut result = select | 0xC0; // Upper bits always set
            match select {
                0x10 => result |= self.joypad_action,   // Bit 4 clear: read action
                0x20 => result |= self.joypad_direction, // Bit 5 clear: read direction
                0x00 => result |= self.joypad_action & self.joypad_direction, // Both selected
                _ => result |= 0x0F,                     // Neither selected
            }
            return result;
        }

        // ROM and external RAM: delegate to cartridge
        if address <= 0x7FFF || (address >= 0xA000 && address <= 0xBFFF) {
            return self.cartridge.read(address);
        }

        // APU register read masks
        if address >= 0xFF10 && address <= 0xFF3F {
            return self.memory[address as usize] | APU_READ_MASKS[(address - 0xFF10) as usize];
        }

        // Echo RAM: 0xE000-0xFDFF mirrors 0xC000-0xDDFF
        if address >= 0xE000 && address <= 0xFDFF {
            return self.memory[(address - 0x2000) as usize];
        }

        // Unused IO registers return 0xFF on DMG
        if address >= 0xFF4C && address <= 0xFF7F {
            return 0xFF;
        }

        self.memory[address as usize]
    }

    /// Fetch the signed value of an address.
    pub fn fetch_signed(&self, address: u16) -> i8 {
        self.fetch(address) as i8
    }

    /// Fetch two consecutive addresses. The first address is the high byte and the second is the low.
    pub fn fetch_16(&self, address: u16) -> u16 {
        (self.fetch(address + 1) as u16) << 8 | (self.fetch(address) as u16)
    }

    /// Set the value of an address.
    pub fn set(&mut self, value: u8, address: u16) {
        // ROM and external RAM: delegate to cartridge (handles MBC registers)
        if address <= 0x7FFF || (address >= 0xA000 && address <= 0xBFFF) {
            self.cartridge.write(address, value);
            return;
        }

        // JOYP: only bits 4-5 (select) are writable
        if address == 0xFF00 {
            self.memory[0xFF00] = (self.memory[0xFF00] & 0xCF) | (value & 0x30);
            return;
        }

        // Echo RAM: 0xE000-0xFDFF mirrors 0xC000-0xDDFF
        if address >= 0xE000 && address <= 0xFDFF {
            self.memory[(address - 0x2000) as usize] = value;
            return;
        }

        self.memory[address as usize] = value;

        // APU registers: queue write notification for Console to forward to APU
        if address >= 0xFF10 && address <= 0xFF3F {
            self.apu_register_writes.push((address, value));
            return;
        }

        // Serial port
        if address == 0xFF02 && value == 0x81 {
            let character = self.fetch(0xFF01) as char;
            // print!("{character}");
            // Transfer complete: clear bit 7 and set incoming byte to 0xFF (no link partner)
            self.memory[0xFF02] = value & 0x7F;
            self.memory[0xFF01] = 0xFF;
            self.set_if(true, Interrupts::Serial);
            return;
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
                let byte = self.fetch(source_base + i);
                self.memory[0xFE00 + i as usize] = byte;
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

    /// Update joypad button state. Each nibble is active-low (0 = pressed).
    pub fn set_joypad(&mut self, action: u8, direction: u8) {
        self.joypad_action = action & 0x0F;
        self.joypad_direction = direction & 0x0F;
    }

    /// Set a value directly in memory without triggering side effects (APU notifications, etc.)
    pub fn set_raw(&mut self, value: u8, address: u16) {
        self.memory[address as usize] = value;
    }

    pub fn get_div(&mut self) -> u16 {
        self.div
    }

    pub fn set_div(&mut self, value: u16) {
        self.div = value;
    }

    pub fn save_ram(&mut self) {
        self.cartridge.save_ram();
    }

    pub fn save_state(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&self.memory)?;
        w.write_all(&self.div.to_le_bytes())?;
        w.write_all(&[self.joypad_action, self.joypad_direction])?;
        self.cartridge.save_state(w)?;
        Ok(())
    }

    pub fn load_state(&mut self, r: &mut dyn Read) -> std::io::Result<()> {
        r.read_exact(&mut self.memory)?;
        let mut buf = [0u8; 2];
        r.read_exact(&mut buf)?;
        self.div = u16::from_le_bytes(buf);
        let mut joypad = [0u8; 2];
        r.read_exact(&mut joypad)?;
        self.joypad_action = joypad[0];
        self.joypad_direction = joypad[1];
        self.cartridge.load_state(r)?;
        self.apu_register_writes.clear();
        Ok(())
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
