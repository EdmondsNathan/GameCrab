use std::io::{Read, Write};

use crate::emulator::system::{
    components::{display::ppu_mode::PpuMode, ram::Interrupts},
    console::Console,
};

pub(crate) struct SpriteObject {
    x: u8,
    y: u8,
    tile_index: u8,
    attributes: u8,
    oam_index: u8,
}

pub(crate) struct Ppu {
    dots: u16,
    draw_mode: PpuMode,

    oam_sprites: Vec<SpriteObject>,

    framebuffer: [u8; 160 * 144 * 4],

    /// Internal window line counter. Only increments on scanlines where the window was rendered.
    window_line: u8,
    /// Whether the window was rendered on the current scanline.
    window_triggered: bool,
}

impl Default for Ppu {
    fn default() -> Self {
        Ppu {
            dots: 0,
            draw_mode: PpuMode::OamScan,

            oam_sprites: vec![],

            framebuffer: [0; 160 * 144 * 4],

            window_line: 0,
            window_triggered: false,
        }
    }
}

impl Ppu {
    pub(crate) fn new() -> Ppu {
        Self::default()
    }

    pub(crate) fn save_state(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&self.dots.to_le_bytes())?;
        w.write_all(&[self.draw_mode.clone() as u8])?;
        w.write_all(&[self.window_line, self.window_triggered as u8])?;
        w.write_all(&self.framebuffer)?;
        Ok(())
    }

    pub(crate) fn load_state(&mut self, r: &mut dyn Read) -> std::io::Result<()> {
        let mut buf = [0u8; 2];
        r.read_exact(&mut buf)?;
        self.dots = u16::from_le_bytes(buf);

        let mut mode = [0u8; 1];
        r.read_exact(&mut mode)?;
        self.draw_mode = match mode[0] {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::OamScan,
            3 => PpuMode::Draw,
            _ => PpuMode::HBlank,
        };

        let mut win = [0u8; 2];
        r.read_exact(&mut win)?;
        self.window_line = win[0];
        self.window_triggered = win[1] != 0;

        r.read_exact(&mut self.framebuffer)?;

        self.oam_sprites.clear();
        Ok(())
    }
}

impl Console {
    pub(crate) fn tick_ppu(&mut self) {
        // println!(
        //     "Dots: {} VBlank IF: {} VBlank IE: {}",
        //     self.ppu.dots,
        //     self.ram.get_if(Interrupts::VBlank),
        //     self.ram.get_ie(Interrupts::VBlank),
        // );

        // Check if LCD is enabled (bit 7 of LCDC)
        if self.get_lcd_control() & 0x80 == 0 {
            // LCD is off - reset PPU state and show white screen
            self.ppu.dots = 0;
            self.set_lcd_y(0);
            self.ppu.draw_mode = PpuMode::HBlank;
            // Clear mode bits in STAT register (bits 0-1)
            self.set_lcd_status(self.get_lcd_status() & 0xFC);
            // Fill framebuffer with white
            self.ppu.framebuffer.fill(255);
            return;
        }

        if self.ppu.dots == 0 {
            self.ppu.oam_sprites.clear();
        }

        self.ppu.dots += 1;

        match self.ppu.draw_mode {
            PpuMode::OamScan => {
                self.oam_scan();

                if self.ppu.dots == 79 {
                    self.set_ppu_mode(PpuMode::Draw);
                }
            }
            PpuMode::Draw => {
                // Starts at 80 so we need to offset it by 80
                let pixel_x = (self.ppu.dots - 80) as u8;
                if pixel_x < 160 {
                    self.draw_background(pixel_x);
                    self.draw_sprites(pixel_x);
                }

                // Can actually be 168-291 dots
                if self.ppu.dots == 251 {
                    self.set_ppu_mode(PpuMode::HBlank);
                }
            }
            PpuMode::HBlank => {
                if self.ppu.dots == 456 {
                    if self.ppu.window_triggered {
                        self.ppu.window_line += 1;
                        self.ppu.window_triggered = false;
                    }

                    self.set_lcd_y(self.get_lcd_y() + 1);
                    self.check_lyc();
                    self.ppu.dots = 0;
                    if self.get_lcd_y() == 144 {
                        self.set_ppu_mode(PpuMode::VBlank);
                        self.ram.set_if(true, Interrupts::VBlank);
                    } else {
                        self.ppu.dots = 0;
                        self.set_ppu_mode(PpuMode::OamScan);
                    }
                }
            }
            PpuMode::VBlank => {
                if self.ppu.dots == 456 {
                    self.ppu.dots = 0;
                    self.set_lcd_y(self.get_lcd_y() + 1);
                    self.check_lyc();
                    if self.get_lcd_y() == 154 {
                        self.set_lcd_y(0);
                        self.check_lyc();
                        self.ppu.window_line = 0;
                        self.set_ppu_mode(PpuMode::OamScan);
                    }
                }
            }
        }
    }

    fn oam_scan(&mut self) {
        // Only execute every other T cycle
        if self.ppu.dots % 2 == 1 {
            return;
        }

        if self.ppu.oam_sprites.len() == 10 {
            return;
        }

        let index = ((self.ppu.dots / 2) as u8) - 1;
        // Sprites start at FFE0 and are 4 bytes each
        let address = 0xFE00 + (index as u16) * 4;

        let sprite_height = if self.get_lcd_control() & 0x04 != 0 {
            16
        } else {
            8
        };

        // let sprite_object = SpriteObject {
        //     y: self.ram.fetch(address),
        //     x: self.ram.fetch(address + 1),
        //     tile_index: self.ram.fetch(address + 2),
        //     attributes: self.ram.fetch(address + 3),
        //     oam_index: index,
        // };

        let sprite_top = self.ram.fetch(address).wrapping_sub(16);
        let is_on_line = self.get_lcd_y() >= sprite_top
            && self.get_lcd_y() < sprite_top.wrapping_add(sprite_height);
        if is_on_line {
            self.ppu.oam_sprites.push(SpriteObject {
                y: self.ram.fetch(address),
                x: self.ram.fetch(address + 1),
                tile_index: self.ram.fetch(address + 2),
                attributes: self.ram.fetch(address + 3),
                oam_index: index,
            });
        }
    }

    fn draw_background(&mut self, pixel_x: u8) {
        let lcdc = self.get_lcd_control();
        let ly = self.get_lcd_y();

        // Check if the window covers this pixel
        let wy = self.get_window_y();
        let wx = self.get_window_x().wrapping_sub(7);
        let window_enabled = lcdc & 0x20 != 0;
        let in_window = window_enabled && ly >= wy && pixel_x >= wx;

        let (tile_x, tile_y, tile_pixel_x, tile_pixel_y, tilemap_base) = if in_window {
            self.ppu.window_triggered = true;

            let win_x = pixel_x - wx;
            let win_y = self.ppu.window_line;

            (
                (win_x / 8) as u16,
                (win_y / 8) as u16,
                win_x % 8,
                win_y % 8,
                // Window tilemap selected by LCDC bit 6
                if lcdc & 0x40 != 0 { 0x9C00u16 } else { 0x9800u16 },
            )
        } else {
            let map_x = self.get_scroll_x().wrapping_add(pixel_x);
            let map_y = self.get_scroll_y().wrapping_add(ly);

            (
                (map_x / 8) as u16,
                (map_y / 8) as u16,
                map_x % 8,
                map_y % 8,
                // BG tilemap selected by LCDC bit 3
                if lcdc & 0x08 != 0 { 0x9C00u16 } else { 0x9800u16 },
            )
        };

        let tilemap_address = tilemap_base + (tile_y * 32) + tile_x;
        let tile_index = self.ram.fetch(tilemap_address);

        let tile_data_base = if lcdc & 0x10 != 0 {
            0x8000 + (tile_index as u16) * 16
        } else {
            0x9000u16.wrapping_add(((tile_index as i8) as i16 * 16) as u16)
        };

        let tile_row_address = tile_data_base + (tile_pixel_y as u16) * 2;
        let byte1 = self.ram.fetch(tile_row_address);
        let byte2 = self.ram.fetch(tile_row_address + 1);

        let bit_position = 7 - tile_pixel_x;
        let color_bit_0 = (byte1 >> bit_position) & 1;
        let color_bit_1 = (byte2 >> bit_position) & 1;
        let color_id = (color_bit_1 << 1) | color_bit_0;

        let palette = self.get_bg_palette();
        let palette_color = (palette >> (color_id * 2)) & 0x03;

        let shade = match palette_color {
            0 => 255,
            1 => 192,
            2 => 96,
            3 => 0,
            _ => 0,
        };

        let fb_index = ((ly as usize) * 160 + (pixel_x as usize)) * 4;
        self.ppu.framebuffer[fb_index] = shade;
        self.ppu.framebuffer[fb_index + 1] = shade;
        self.ppu.framebuffer[fb_index + 2] = shade;
        self.ppu.framebuffer[fb_index + 3] = 255;
    }

    fn draw_sprites(&mut self, pixel_x: u8) {
        // Check if sprites are enabled (LCDC bit 1)
        if self.get_lcd_control() & 0x02 == 0 {
            return;
        }

        let sprite_height: u8 = if self.get_lcd_control() & 0x04 != 0 {
            16
        } else {
            8
        };

        let ly = self.get_lcd_y();

        // Iterate sprites in reverse so lower-index sprites (higher priority) draw last
        for i in (0..self.ppu.oam_sprites.len()).rev() {
            let sprite_x = self.ppu.oam_sprites[i].x.wrapping_sub(8);
            let sprite_y = self.ppu.oam_sprites[i].y.wrapping_sub(16);
            let tile_index = self.ppu.oam_sprites[i].tile_index;
            let attributes = self.ppu.oam_sprites[i].attributes;

            // Check if this sprite covers pixel_x
            if pixel_x < sprite_x || pixel_x >= sprite_x.wrapping_add(8) {
                continue;
            }

            let x_flip = attributes & 0x20 != 0;
            let y_flip = attributes & 0x40 != 0;
            let bg_priority = attributes & 0x80 != 0;
            let palette_num = attributes & 0x10 != 0;

            let mut tile_row = ly.wrapping_sub(sprite_y) as u16;
            if y_flip {
                tile_row = (sprite_height as u16 - 1) - tile_row;
            }

            let actual_tile = if sprite_height == 16 {
                // 8x16 mode: bit 0 of tile index is ignored
                if tile_row < 8 {
                    tile_index & 0xFE
                } else {
                    tile_row -= 8;
                    tile_index | 0x01
                }
            } else {
                tile_index
            };

            let tile_data_address = 0x8000 + (actual_tile as u16) * 16 + tile_row * 2;
            let byte1 = self.ram.fetch(tile_data_address);
            let byte2 = self.ram.fetch(tile_data_address + 1);

            let mut tile_pixel_x = pixel_x.wrapping_sub(sprite_x);
            if x_flip {
                tile_pixel_x = 7 - tile_pixel_x;
            }

            let bit_position = 7 - tile_pixel_x;
            let color_bit_0 = (byte1 >> bit_position) & 1;
            let color_bit_1 = (byte2 >> bit_position) & 1;
            let color_id = (color_bit_1 << 1) | color_bit_0;

            // Color 0 is transparent for sprites
            if color_id == 0 {
                continue;
            }

            // If BG priority bit is set, sprite is behind non-zero BG colors
            if bg_priority {
                let fb_index =
                    ((ly as usize) * 160 + (pixel_x as usize)) * 4;
                // Check if BG pixel is non-white (non-zero color)
                if self.ppu.framebuffer[fb_index] != 255 {
                    continue;
                }
            }

            let palette = if palette_num {
                self.get_obj_palette1()
            } else {
                self.get_obj_palette0()
            };
            let palette_color = (palette >> (color_id * 2)) & 0x03;

            let shade = match palette_color {
                0 => 255,
                1 => 192,
                2 => 96,
                3 => 0,
                _ => 0,
            };

            let fb_index = ((ly as usize) * 160 + (pixel_x as usize)) * 4;
            self.ppu.framebuffer[fb_index] = shade;
            self.ppu.framebuffer[fb_index + 1] = shade;
            self.ppu.framebuffer[fb_index + 2] = shade;
            self.ppu.framebuffer[fb_index + 3] = 255;
        }
    }

    fn set_ppu_mode(&mut self, new_mode: PpuMode) {
        self.ppu.draw_mode = new_mode.clone();
        self.set_lcd_status(self.get_lcd_status() & 0xFC | (new_mode.clone()) as u8);

        let stat = self.get_lcd_status();
        let fire = match new_mode {
            PpuMode::HBlank => stat & 0x08 != 0,
            PpuMode::VBlank => stat & 0x10 != 0,
            PpuMode::OamScan => stat & 0x20 != 0,
            PpuMode::Draw => false,
        };
        if fire {
            self.ram.set_if(true, Interrupts::Lcd);
        }
    }

    fn check_lyc(&mut self) {
        let ly = self.get_lcd_y();
        let lyc = self.get_lcd_y_compare();
        let mut stat = self.get_lcd_status();

        if ly == lyc {
            stat |= 0x04; // Set bit 2 (coincidence flag)
            self.set_lcd_status(stat);
            if stat & 0x40 != 0 {
                self.ram.set_if(true, Interrupts::Lcd);
            }
        } else {
            stat &= !0x04; // Clear bit 2
            self.set_lcd_status(stat);
        }
    }

    fn get_lcd_control(&self) -> u8 {
        self.ram.fetch(0xFF40)
    }

    fn set_lcd_control(&mut self, value: u8) {
        self.ram.set(value, 0xFF40);
    }

    fn get_lcd_status(&self) -> u8 {
        self.ram.fetch(0xFF41)
    }

    fn set_lcd_status(&mut self, value: u8) {
        self.ram.set(value, 0xFF41);
    }

    fn get_scroll_y(&self) -> u8 {
        self.ram.fetch(0xFF42)
    }

    fn set_scroll_y(&mut self, value: u8) {
        self.ram.set(value, 0xFF42);
    }

    fn get_scroll_x(&self) -> u8 {
        self.ram.fetch(0xFF43)
    }

    fn set_scroll_x(&mut self, value: u8) {
        self.ram.set(value, 0xFF43);
    }

    fn get_lcd_y(&self) -> u8 {
        self.ram.fetch(0xFF44)
    }

    fn set_lcd_y(&mut self, value: u8) {
        self.ram.set(value, 0xFF44);
    }

    fn get_lcd_y_compare(&self) -> u8 {
        self.ram.fetch(0xFF45)
    }

    fn set_lcd_y_compare(&mut self, value: u8) {
        self.ram.set(value, 0xFF45);
    }

    fn get_dma(&self) -> u8 {
        self.ram.fetch(0xFF46)
    }

    fn set_dma(&mut self, value: u8) {
        self.ram.set(value, 0xFF46);
    }

    fn get_bg_palette(&self) -> u8 {
        self.ram.fetch(0xFF47)
    }

    fn set_bg_palette(&mut self, value: u8) {
        self.ram.set(value, 0xFF47);
    }

    fn get_obj_palette0(&self) -> u8 {
        self.ram.fetch(0xFF48)
    }

    fn set_obj_palette0(&mut self, value: u8) {
        self.ram.set(value, 0xFF48);
    }

    fn get_obj_palette1(&self) -> u8 {
        self.ram.fetch(0xFF49)
    }

    fn set_obj_palette1(&mut self, value: u8) {
        self.ram.set(value, 0xFF49);
    }

    fn get_window_y(&self) -> u8 {
        self.ram.fetch(0xFF4A)
    }

    fn set_window_y(&mut self, value: u8) {
        self.ram.set(value, 0xFF4A);
    }

    fn get_window_x(&self) -> u8 {
        self.ram.fetch(0xFF4B)
    }

    fn set_window_x(&mut self, value: u8) {
        self.ram.set(value, 0xFF4B);
    }

    pub(crate) fn get_framebuffer(&self) -> [u8; 160 * 144 * 4] {
        self.ppu.framebuffer
    }
}
