use std::io::{Read, Write};

use ringbuf::traits::Producer;
use ringbuf::HeapProd;

use crate::emulator::system::console::Console;

const CPU_CLOCK: u32 = 4_194_304;
const SAMPLE_RATE: u32 = 44_100;
// Use fixed-point (16.16) accumulator to avoid integer-division drift.
// Each T-cycle adds SAMPLE_RATE; when accumulator >= CPU_CLOCK we emit a sample.
// This gives exact long-term sample rate with no fractional error buildup.

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
    [1, 0, 0, 0, 0, 0, 0, 1], // 25%
    [1, 0, 0, 0, 0, 1, 1, 1], // 50%
    [0, 1, 1, 1, 1, 1, 1, 0], // 75%
];

// OR masks for reading write-only register bits
const NR_READ_MASKS: [u8; 48] = [
    0x80, // 0xFF10 NR10: sweep
    0x3F, // 0xFF11 NR11: duty (bits 6-7 readable, length write-only)
    0x00, // 0xFF12 NR12: envelope (all readable)
    0xFF, // 0xFF13 NR13: freq low (write-only)
    0xBF, // 0xFF14 NR14: freq high (bit 6 readable, rest write-only)
    0xFF, // 0xFF15 unused
    0x3F, // 0xFF16 NR21: duty
    0x00, // 0xFF17 NR22: envelope
    0xFF, // 0xFF18 NR23: freq low
    0xBF, // 0xFF19 NR24: freq high
    0x7F, // 0xFF1A NR30: DAC enable (bit 7 readable)
    0xFF, // 0xFF1B NR31: length (write-only)
    0x9F, // 0xFF1C NR32: volume code (bits 5-6 readable)
    0xFF, // 0xFF1D NR33: freq low (write-only)
    0xBF, // 0xFF1E NR34: freq high
    0xFF, // 0xFF1F unused
    0xFF, // 0xFF20 NR41: length (write-only)
    0x00, // 0xFF21 NR42: envelope
    0x00, // 0xFF22 NR43: polynomial counter
    0xBF, // 0xFF23 NR44: control
    0x00, // 0xFF24 NR50: master volume
    0x00, // 0xFF25 NR51: panning
    0x70, // 0xFF26 NR52: status (handled specially, bits 4-6 always 1)
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 0xFF27-0xFF2F unused
    // 0xFF30-0xFF3F wave RAM (all readable, handled separately)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub(crate) struct SquareChannel {
    enabled: bool,
    dac_enabled: bool,
    length_counter: u16,
    length_enabled: bool,
    volume: u8,
    envelope_period: u8,
    envelope_direction: bool, // true = increase
    envelope_timer: u8,
    initial_volume: u8,
    frequency: u16,
    frequency_timer: u16,
    duty: u8,
    duty_position: u8,
    // Sweep (channel 1 only)
    sweep_period: u8,
    sweep_direction: bool, // true = subtract
    sweep_shift: u8,
    sweep_timer: u8,
    sweep_enabled: bool,
    shadow_frequency: u16,
    has_sweep: bool,
}

impl SquareChannel {
    fn new(has_sweep: bool) -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            length_counter: 0,
            length_enabled: false,
            volume: 0,
            envelope_period: 0,
            envelope_direction: false,
            envelope_timer: 0,
            initial_volume: 0,
            frequency: 0,
            frequency_timer: 0,
            duty: 0,
            duty_position: 0,
            sweep_period: 0,
            sweep_direction: false,
            sweep_shift: 0,
            sweep_timer: 0,
            sweep_enabled: false,
            shadow_frequency: 0,
            has_sweep,
        }
    }

    fn tick(&mut self) {
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.frequency) * 4;
            self.duty_position = (self.duty_position + 1) & 7;
        }
        self.frequency_timer -= 1;
    }

    fn output(&self) -> f32 {
        if !self.enabled || !self.dac_enabled {
            return 0.0;
        }
        let sample = DUTY_TABLE[self.duty as usize][self.duty_position as usize];
        let dac_input = if sample != 0 { self.volume } else { 0 };
        // DAC converts 0-15 to -1.0..+1.0
        (dac_input as f32 / 7.5) - 1.0
    }

    fn tick_length(&mut self) {
        if self.length_enabled && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    fn tick_envelope(&mut self) {
        if self.envelope_period == 0 {
            return;
        }
        if self.envelope_timer > 0 {
            self.envelope_timer -= 1;
        }
        if self.envelope_timer == 0 {
            self.envelope_timer = self.envelope_period;
            if self.envelope_direction && self.volume < 15 {
                self.volume += 1;
            } else if !self.envelope_direction && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }

    fn tick_sweep(&mut self) {
        if !self.has_sweep {
            return;
        }
        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }
        if self.sweep_timer == 0 {
            self.sweep_timer = if self.sweep_period != 0 {
                self.sweep_period
            } else {
                8
            };
            if self.sweep_enabled && self.sweep_period != 0 {
                let new_freq = self.calculate_sweep();
                if new_freq <= 2047 && self.sweep_shift != 0 {
                    self.frequency = new_freq;
                    self.shadow_frequency = new_freq;
                    // Overflow check again
                    self.calculate_sweep();
                }
            }
        }
    }

    fn calculate_sweep(&mut self) -> u16 {
        let offset = self.shadow_frequency >> self.sweep_shift;
        let new_freq = if self.sweep_direction {
            self.shadow_frequency.wrapping_sub(offset)
        } else {
            self.shadow_frequency + offset
        };
        if new_freq > 2047 {
            self.enabled = false;
        }
        new_freq
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_counter == 0 {
            self.length_counter = 64;
        }
        self.frequency_timer = (2048 - self.frequency) * 4;
        self.volume = self.initial_volume;
        self.envelope_timer = self.envelope_period;

        // Sweep init (channel 1 only)
        if self.has_sweep {
            self.shadow_frequency = self.frequency;
            self.sweep_timer = if self.sweep_period != 0 {
                self.sweep_period
            } else {
                8
            };
            self.sweep_enabled = self.sweep_period != 0 || self.sweep_shift != 0;
            if self.sweep_shift != 0 {
                self.calculate_sweep();
            }
        }

        if !self.dac_enabled {
            self.enabled = false;
        }
    }

    fn save_state(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&[
            self.enabled as u8,
            self.dac_enabled as u8,
            self.volume,
            self.envelope_period,
            self.envelope_direction as u8,
            self.envelope_timer,
            self.initial_volume,
            self.duty,
            self.duty_position,
            self.sweep_period,
            self.sweep_direction as u8,
            self.sweep_shift,
            self.sweep_timer,
            self.sweep_enabled as u8,
            self.length_enabled as u8,
            self.has_sweep as u8,
        ])?;
        w.write_all(&self.length_counter.to_le_bytes())?;
        w.write_all(&self.frequency.to_le_bytes())?;
        w.write_all(&self.frequency_timer.to_le_bytes())?;
        w.write_all(&self.shadow_frequency.to_le_bytes())?;
        Ok(())
    }

    fn load_state(&mut self, r: &mut dyn Read) -> std::io::Result<()> {
        let mut buf = [0u8; 16];
        r.read_exact(&mut buf)?;
        self.enabled = buf[0] != 0;
        self.dac_enabled = buf[1] != 0;
        self.volume = buf[2];
        self.envelope_period = buf[3];
        self.envelope_direction = buf[4] != 0;
        self.envelope_timer = buf[5];
        self.initial_volume = buf[6];
        self.duty = buf[7];
        self.duty_position = buf[8];
        self.sweep_period = buf[9];
        self.sweep_direction = buf[10] != 0;
        self.sweep_shift = buf[11];
        self.sweep_timer = buf[12];
        self.sweep_enabled = buf[13] != 0;
        self.length_enabled = buf[14] != 0;
        self.has_sweep = buf[15] != 0;

        let mut buf16 = [0u8; 2];
        r.read_exact(&mut buf16)?;
        self.length_counter = u16::from_le_bytes(buf16);
        r.read_exact(&mut buf16)?;
        self.frequency = u16::from_le_bytes(buf16);
        r.read_exact(&mut buf16)?;
        self.frequency_timer = u16::from_le_bytes(buf16);
        r.read_exact(&mut buf16)?;
        self.shadow_frequency = u16::from_le_bytes(buf16);
        Ok(())
    }
}

pub(crate) struct WaveChannel {
    enabled: bool,
    dac_enabled: bool,
    length_counter: u16,
    length_enabled: bool,
    volume_code: u8, // 0=mute, 1=100%, 2=50%, 3=25%
    frequency: u16,
    frequency_timer: u16,
    position: u8, // 0-31
    wave_ram: [u8; 16],
}

impl WaveChannel {
    fn new() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            length_counter: 0,
            length_enabled: false,
            volume_code: 0,
            frequency: 0,
            frequency_timer: 0,
            position: 0,
            wave_ram: [0; 16],
        }
    }

    fn tick(&mut self) {
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.frequency) * 2;
            self.position = (self.position + 1) & 31;
        }
        self.frequency_timer -= 1;
    }

    fn output(&self) -> f32 {
        if !self.enabled || !self.dac_enabled {
            return 0.0;
        }
        let byte = self.wave_ram[(self.position / 2) as usize];
        let sample = if self.position % 2 == 0 {
            (byte >> 4) & 0x0F
        } else {
            byte & 0x0F
        };
        let shifted = match self.volume_code {
            0 => 0,
            1 => sample,
            2 => sample >> 1,
            3 => sample >> 2,
            _ => 0,
        };
        (shifted as f32 / 7.5) - 1.0
    }

    fn tick_length(&mut self) {
        if self.length_enabled && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_counter == 0 {
            self.length_counter = 256;
        }
        self.frequency_timer = (2048 - self.frequency) * 2;
        self.position = 0;
        if !self.dac_enabled {
            self.enabled = false;
        }
    }

    fn save_state(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&[
            self.enabled as u8,
            self.dac_enabled as u8,
            self.volume_code,
            self.position,
            self.length_enabled as u8,
        ])?;
        w.write_all(&self.length_counter.to_le_bytes())?;
        w.write_all(&self.frequency.to_le_bytes())?;
        w.write_all(&self.frequency_timer.to_le_bytes())?;
        w.write_all(&self.wave_ram)?;
        Ok(())
    }

    fn load_state(&mut self, r: &mut dyn Read) -> std::io::Result<()> {
        let mut buf = [0u8; 5];
        r.read_exact(&mut buf)?;
        self.enabled = buf[0] != 0;
        self.dac_enabled = buf[1] != 0;
        self.volume_code = buf[2];
        self.position = buf[3];
        self.length_enabled = buf[4] != 0;

        let mut buf16 = [0u8; 2];
        r.read_exact(&mut buf16)?;
        self.length_counter = u16::from_le_bytes(buf16);
        r.read_exact(&mut buf16)?;
        self.frequency = u16::from_le_bytes(buf16);
        r.read_exact(&mut buf16)?;
        self.frequency_timer = u16::from_le_bytes(buf16);
        r.read_exact(&mut self.wave_ram)?;
        Ok(())
    }
}

pub(crate) struct NoiseChannel {
    enabled: bool,
    dac_enabled: bool,
    length_counter: u16,
    length_enabled: bool,
    volume: u8,
    envelope_period: u8,
    envelope_direction: bool,
    envelope_timer: u8,
    initial_volume: u8,
    clock_shift: u8,
    width_mode: bool, // true = 7-bit, false = 15-bit
    divisor_code: u8,
    frequency_timer: u16,
    lfsr: u16,
}

impl NoiseChannel {
    fn new() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            length_counter: 0,
            length_enabled: false,
            volume: 0,
            envelope_period: 0,
            envelope_direction: false,
            envelope_timer: 0,
            initial_volume: 0,
            clock_shift: 0,
            width_mode: false,
            divisor_code: 0,
            frequency_timer: 0,
            lfsr: 0x7FFF,
        }
    }

    fn get_divisor(&self) -> u16 {
        match self.divisor_code {
            0 => 8,
            n => (n as u16) * 16,
        }
    }

    fn tick(&mut self) {
        if self.frequency_timer == 0 {
            self.frequency_timer = self.get_divisor() << self.clock_shift;
            let xor_result = (self.lfsr & 1) ^ ((self.lfsr >> 1) & 1);
            self.lfsr = (self.lfsr >> 1) | (xor_result << 14);
            if self.width_mode {
                self.lfsr &= !(1 << 6);
                self.lfsr |= xor_result << 6;
            }
        }
        self.frequency_timer -= 1;
    }

    fn output(&self) -> f32 {
        if !self.enabled || !self.dac_enabled {
            return 0.0;
        }
        let bit = (!self.lfsr) & 1;
        let dac_input = if bit != 0 { self.volume } else { 0 };
        (dac_input as f32 / 7.5) - 1.0
    }

    fn tick_length(&mut self) {
        if self.length_enabled && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    fn tick_envelope(&mut self) {
        if self.envelope_period == 0 {
            return;
        }
        if self.envelope_timer > 0 {
            self.envelope_timer -= 1;
        }
        if self.envelope_timer == 0 {
            self.envelope_timer = self.envelope_period;
            if self.envelope_direction && self.volume < 15 {
                self.volume += 1;
            } else if !self.envelope_direction && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_counter == 0 {
            self.length_counter = 64;
        }
        self.frequency_timer = self.get_divisor() << self.clock_shift;
        self.volume = self.initial_volume;
        self.envelope_timer = self.envelope_period;
        self.lfsr = 0x7FFF;
        if !self.dac_enabled {
            self.enabled = false;
        }
    }

    fn save_state(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&[
            self.enabled as u8,
            self.dac_enabled as u8,
            self.volume,
            self.envelope_period,
            self.envelope_direction as u8,
            self.envelope_timer,
            self.initial_volume,
            self.clock_shift,
            self.width_mode as u8,
            self.divisor_code,
            self.length_enabled as u8,
        ])?;
        w.write_all(&self.length_counter.to_le_bytes())?;
        w.write_all(&self.frequency_timer.to_le_bytes())?;
        w.write_all(&self.lfsr.to_le_bytes())?;
        Ok(())
    }

    fn load_state(&mut self, r: &mut dyn Read) -> std::io::Result<()> {
        let mut buf = [0u8; 11];
        r.read_exact(&mut buf)?;
        self.enabled = buf[0] != 0;
        self.dac_enabled = buf[1] != 0;
        self.volume = buf[2];
        self.envelope_period = buf[3];
        self.envelope_direction = buf[4] != 0;
        self.envelope_timer = buf[5];
        self.initial_volume = buf[6];
        self.clock_shift = buf[7];
        self.width_mode = buf[8] != 0;
        self.divisor_code = buf[9];
        self.length_enabled = buf[10] != 0;

        let mut buf16 = [0u8; 2];
        r.read_exact(&mut buf16)?;
        self.length_counter = u16::from_le_bytes(buf16);
        r.read_exact(&mut buf16)?;
        self.frequency_timer = u16::from_le_bytes(buf16);
        r.read_exact(&mut buf16)?;
        self.lfsr = u16::from_le_bytes(buf16);
        Ok(())
    }
}

pub(crate) struct Apu {
    pub(crate) enabled: bool,
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    frame_sequencer_step: u8,
    frame_sequencer_counter: u16,
    // Accumulator-based sample timing: add SAMPLE_RATE each tick,
    // emit a sample when it reaches CPU_CLOCK. This avoids integer
    // division drift that causes crackling.
    sample_accumulator: u32,
    // Registers stored locally
    nr50: u8, // Master volume
    nr51: u8, // Panning
    // Ring buffer producer for audio output
    producer: Option<HeapProd<f32>>,
}

impl Default for Apu {
    fn default() -> Self {
        Self {
            enabled: false,
            ch1: SquareChannel::new(true),
            ch2: SquareChannel::new(false),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            frame_sequencer_step: 0,
            frame_sequencer_counter: 0,
            sample_accumulator: 0,
            nr50: 0,
            nr51: 0,
            producer: None,
        }
    }
}

impl Apu {
    pub(crate) fn set_producer(&mut self, producer: HeapProd<f32>) {
        self.producer = Some(producer);
    }

    pub(crate) fn read_register(&self, address: u16) -> u8 {
        let reg_index = (address - 0xFF10) as usize;

        // NR52 is special: returns master enable + channel status
        if address == 0xFF26 {
            let status = (self.enabled as u8) << 7
                | 0x70 // bits 4-6 always set
                | (self.ch4.enabled as u8) << 3
                | (self.ch3.enabled as u8) << 2
                | (self.ch2.enabled as u8) << 1
                | (self.ch1.enabled as u8);
            return status;
        }

        // Wave RAM is fully readable
        if address >= 0xFF30 && address <= 0xFF3F {
            return self.ch3.wave_ram[(address - 0xFF30) as usize];
        }

        // Unused addresses in the APU range
        if address > 0xFF26 && address < 0xFF30 {
            return 0xFF;
        }

        // Return register value OR'd with write-only bit mask
        let value = match address {
            0xFF10 => {
                (self.ch1.sweep_period << 4)
                    | ((self.ch1.sweep_direction as u8) << 3)
                    | self.ch1.sweep_shift
            }
            0xFF11 => self.ch1.duty << 6,
            0xFF12 => {
                (self.ch1.initial_volume << 4)
                    | ((self.ch1.envelope_direction as u8) << 3)
                    | self.ch1.envelope_period
            }
            0xFF14 => (self.ch1.length_enabled as u8) << 6,
            0xFF16 => self.ch2.duty << 6,
            0xFF17 => {
                (self.ch2.initial_volume << 4)
                    | ((self.ch2.envelope_direction as u8) << 3)
                    | self.ch2.envelope_period
            }
            0xFF19 => (self.ch2.length_enabled as u8) << 6,
            0xFF1A => (self.ch3.dac_enabled as u8) << 7,
            0xFF1C => self.ch3.volume_code << 5,
            0xFF1E => (self.ch3.length_enabled as u8) << 6,
            0xFF21 => {
                (self.ch4.initial_volume << 4)
                    | ((self.ch4.envelope_direction as u8) << 3)
                    | self.ch4.envelope_period
            }
            0xFF22 => {
                (self.ch4.clock_shift << 4)
                    | ((self.ch4.width_mode as u8) << 3)
                    | self.ch4.divisor_code
            }
            0xFF23 => (self.ch4.length_enabled as u8) << 6,
            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            _ => 0x00,
        };

        if reg_index < NR_READ_MASKS.len() {
            value | NR_READ_MASKS[reg_index]
        } else {
            value
        }
    }

    pub(crate) fn write_register(&mut self, address: u16, value: u8) {
        // If APU is off, only NR52 and wave RAM can be written
        if !self.enabled && address != 0xFF26 && !(address >= 0xFF30 && address <= 0xFF3F) {
            // Exception: length counters can still be written on DMG
            match address {
                0xFF11 => self.ch1.length_counter = 64 - (value & 0x3F) as u16,
                0xFF16 => self.ch2.length_counter = 64 - (value & 0x3F) as u16,
                0xFF1B => self.ch3.length_counter = 256 - value as u16,
                0xFF20 => self.ch4.length_counter = 64 - (value & 0x3F) as u16,
                _ => {}
            }
            return;
        }

        match address {
            // Channel 1 - Square with sweep
            0xFF10 => {
                self.ch1.sweep_period = (value >> 4) & 0x07;
                self.ch1.sweep_direction = value & 0x08 != 0;
                self.ch1.sweep_shift = value & 0x07;
            }
            0xFF11 => {
                self.ch1.duty = (value >> 6) & 0x03;
                self.ch1.length_counter = 64 - (value & 0x3F) as u16;
            }
            0xFF12 => {
                self.ch1.initial_volume = (value >> 4) & 0x0F;
                self.ch1.envelope_direction = value & 0x08 != 0;
                self.ch1.envelope_period = value & 0x07;
                self.ch1.dac_enabled = value & 0xF8 != 0;
                if !self.ch1.dac_enabled {
                    self.ch1.enabled = false;
                }
            }
            0xFF13 => {
                self.ch1.frequency = (self.ch1.frequency & 0x700) | value as u16;
            }
            0xFF14 => {
                self.ch1.frequency =
                    (self.ch1.frequency & 0xFF) | (((value & 0x07) as u16) << 8);
                self.ch1.length_enabled = value & 0x40 != 0;
                if value & 0x80 != 0 {
                    self.ch1.trigger();
                }
            }

            // Channel 2 - Square
            0xFF16 => {
                self.ch2.duty = (value >> 6) & 0x03;
                self.ch2.length_counter = 64 - (value & 0x3F) as u16;
            }
            0xFF17 => {
                self.ch2.initial_volume = (value >> 4) & 0x0F;
                self.ch2.envelope_direction = value & 0x08 != 0;
                self.ch2.envelope_period = value & 0x07;
                self.ch2.dac_enabled = value & 0xF8 != 0;
                if !self.ch2.dac_enabled {
                    self.ch2.enabled = false;
                }
            }
            0xFF18 => {
                self.ch2.frequency = (self.ch2.frequency & 0x700) | value as u16;
            }
            0xFF19 => {
                self.ch2.frequency =
                    (self.ch2.frequency & 0xFF) | (((value & 0x07) as u16) << 8);
                self.ch2.length_enabled = value & 0x40 != 0;
                if value & 0x80 != 0 {
                    self.ch2.trigger();
                }
            }

            // Channel 3 - Wave
            0xFF1A => {
                self.ch3.dac_enabled = value & 0x80 != 0;
                if !self.ch3.dac_enabled {
                    self.ch3.enabled = false;
                }
            }
            0xFF1B => {
                self.ch3.length_counter = 256 - value as u16;
            }
            0xFF1C => {
                self.ch3.volume_code = (value >> 5) & 0x03;
            }
            0xFF1D => {
                self.ch3.frequency = (self.ch3.frequency & 0x700) | value as u16;
            }
            0xFF1E => {
                self.ch3.frequency =
                    (self.ch3.frequency & 0xFF) | (((value & 0x07) as u16) << 8);
                self.ch3.length_enabled = value & 0x40 != 0;
                if value & 0x80 != 0 {
                    self.ch3.trigger();
                }
            }

            // Channel 4 - Noise
            0xFF20 => {
                self.ch4.length_counter = 64 - (value & 0x3F) as u16;
            }
            0xFF21 => {
                self.ch4.initial_volume = (value >> 4) & 0x0F;
                self.ch4.envelope_direction = value & 0x08 != 0;
                self.ch4.envelope_period = value & 0x07;
                self.ch4.dac_enabled = value & 0xF8 != 0;
                if !self.ch4.dac_enabled {
                    self.ch4.enabled = false;
                }
            }
            0xFF22 => {
                self.ch4.clock_shift = (value >> 4) & 0x0F;
                self.ch4.width_mode = value & 0x08 != 0;
                self.ch4.divisor_code = value & 0x07;
            }
            0xFF23 => {
                self.ch4.length_enabled = value & 0x40 != 0;
                if value & 0x80 != 0 {
                    self.ch4.trigger();
                }
            }

            // Master controls
            0xFF24 => {
                self.nr50 = value;
            }
            0xFF25 => {
                self.nr51 = value;
            }
            0xFF26 => {
                let was_enabled = self.enabled;
                self.enabled = value & 0x80 != 0;
                if was_enabled && !self.enabled {
                    // Power off: zero all registers
                    self.ch1 = SquareChannel::new(true);
                    self.ch2 = SquareChannel::new(false);
                    self.ch3.enabled = false;
                    self.ch3.dac_enabled = false;
                    self.ch3.length_counter = 0;
                    self.ch3.length_enabled = false;
                    self.ch3.volume_code = 0;
                    self.ch3.frequency = 0;
                    self.ch3.frequency_timer = 0;
                    self.ch3.position = 0;
                    // Wave RAM is preserved on power off
                    self.ch4 = NoiseChannel::new();
                    self.nr50 = 0;
                    self.nr51 = 0;
                    self.frame_sequencer_step = 0;
                }
            }

            // Wave RAM
            0xFF30..=0xFF3F => {
                self.ch3.wave_ram[(address - 0xFF30) as usize] = value;
            }

            _ => {}
        }
    }

    pub(crate) fn tick(&mut self) {
        if !self.enabled {
            self.sample_accumulator += SAMPLE_RATE;
            if self.sample_accumulator >= CPU_CLOCK {
                self.sample_accumulator -= CPU_CLOCK;
                self.push_sample(0.0, 0.0);
            }
            return;
        }

        // Tick frame sequencer (512 Hz = every 8192 T-cycles)
        self.frame_sequencer_counter += 1;
        if self.frame_sequencer_counter >= 8192 {
            self.frame_sequencer_counter = 0;
            match self.frame_sequencer_step {
                0 | 4 => {
                    self.ch1.tick_length();
                    self.ch2.tick_length();
                    self.ch3.tick_length();
                    self.ch4.tick_length();
                }
                2 | 6 => {
                    self.ch1.tick_length();
                    self.ch2.tick_length();
                    self.ch3.tick_length();
                    self.ch4.tick_length();
                    self.ch1.tick_sweep();
                }
                7 => {
                    self.ch1.tick_envelope();
                    self.ch2.tick_envelope();
                    self.ch4.tick_envelope();
                }
                _ => {}
            }
            self.frame_sequencer_step = (self.frame_sequencer_step + 1) & 7;
        }

        // Tick channel frequency timers
        self.ch1.tick();
        self.ch2.tick();
        self.ch3.tick();
        self.ch4.tick();

        // Generate sample using accumulator for exact long-term rate
        self.sample_accumulator += SAMPLE_RATE;
        if self.sample_accumulator >= CPU_CLOCK {
            self.sample_accumulator -= CPU_CLOCK;

            let ch1_out = self.ch1.output();
            let ch2_out = self.ch2.output();
            let ch3_out = self.ch3.output();
            let ch4_out = self.ch4.output();

            let mut left = 0.0f32;
            let mut right = 0.0f32;

            // NR51 panning
            if self.nr51 & 0x10 != 0 {
                left += ch1_out;
            }
            if self.nr51 & 0x20 != 0 {
                left += ch2_out;
            }
            if self.nr51 & 0x40 != 0 {
                left += ch3_out;
            }
            if self.nr51 & 0x80 != 0 {
                left += ch4_out;
            }
            if self.nr51 & 0x01 != 0 {
                right += ch1_out;
            }
            if self.nr51 & 0x02 != 0 {
                right += ch2_out;
            }
            if self.nr51 & 0x04 != 0 {
                right += ch3_out;
            }
            if self.nr51 & 0x08 != 0 {
                right += ch4_out;
            }

            // NR50 master volume (0-7)
            let left_vol = ((self.nr50 >> 4) & 0x07) as f32 + 1.0;
            let right_vol = (self.nr50 & 0x07) as f32 + 1.0;

            left = left * left_vol / 32.0;
            right = right * right_vol / 32.0;

            self.push_sample(left, right);
        }
    }

    fn push_sample(&mut self, left: f32, right: f32) {
        if let Some(ref mut producer) = self.producer {
            // Push stereo pair; drop samples if buffer is full
            let _ = producer.try_push(left);
            let _ = producer.try_push(right);
        }
    }

    pub(crate) fn save_state(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&[
            self.enabled as u8,
            self.frame_sequencer_step,
            self.nr50,
            self.nr51,
        ])?;
        w.write_all(&self.frame_sequencer_counter.to_le_bytes())?;
        w.write_all(&self.sample_accumulator.to_le_bytes())?;
        self.ch1.save_state(w)?;
        self.ch2.save_state(w)?;
        self.ch3.save_state(w)?;
        self.ch4.save_state(w)?;
        Ok(())
    }

    pub(crate) fn load_state(&mut self, r: &mut dyn Read) -> std::io::Result<()> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        self.enabled = buf[0] != 0;
        self.frame_sequencer_step = buf[1];
        self.nr50 = buf[2];
        self.nr51 = buf[3];

        let mut buf16 = [0u8; 2];
        r.read_exact(&mut buf16)?;
        self.frame_sequencer_counter = u16::from_le_bytes(buf16);

        let mut buf32 = [0u8; 4];
        r.read_exact(&mut buf32)?;
        self.sample_accumulator = u32::from_le_bytes(buf32);

        self.ch1.load_state(r)?;
        self.ch2.load_state(r)?;
        self.ch3.load_state(r)?;
        self.ch4.load_state(r)?;
        Ok(())
    }
}

impl Console {
    pub(crate) fn tick_apu(&mut self) {
        // Drain APU register write notifications from RAM
        for i in 0..self.ram.apu_register_writes.len() {
            let (addr, val) = self.ram.apu_register_writes[i];
            self.apu.write_register(addr, val);
        }
        self.ram.apu_register_writes.clear();

        self.apu.tick();

        // Sync NR52 status back to RAM for reads
        let nr52 = self.apu.read_register(0xFF26);
        self.ram.set_raw(nr52, 0xFF26);
    }
}
