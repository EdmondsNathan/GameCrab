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
}

impl Default for Ppu {
    fn default() -> Self {
        Ppu {
            dots: 0,
            draw_mode: PpuMode::OamScan,

            // oam_sprite_index: 0,
            oam_sprites: vec![],

            framebuffer: [0; 160 * 144 * 4],
        }
    }
}

impl Ppu {
    pub(crate) fn new() -> Ppu {
        Self::default()
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
                }

                // Can actually be 168-291 dots
                if self.ppu.dots == 251 {
                    self.set_ppu_mode(PpuMode::HBlank);
                }
            }
            PpuMode::HBlank => {
                if self.ppu.dots == 456 {
                    self.set_lcd_y(self.get_lcd_y() + 1);
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
                    if self.get_lcd_y() == 154 {
                        self.set_lcd_y(0);
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
        let map_x = self.get_scroll_x().wrapping_add(pixel_x);
        let map_y = self.get_scroll_y().wrapping_add(self.get_lcd_y());

        let tile_x = (map_x / 8) as u16;
        let tile_y = (map_y / 8) as u16;

        let tile_pixel_x = map_x % 8;
        let tile_pixel_y = map_y % 8;

        let tilemap_base = if self.get_lcd_control() & 0x08 != 0 {
            0x9C00
        } else {
            0x9800
        };
        let tilemap_address = tilemap_base + (tile_y * 32) + tile_x;
        let tile_index = self.ram.fetch(tilemap_address);

        let tile_data_base = if self.get_lcd_control() & 0x10 != 0 {
            // Unsigned: 0x8000 base
            0x8000 + (tile_index as u16) * 16
        } else {
            // Signed: 0x9000 base, tile_index is i8
            0x9000u16.wrapping_add(((tile_index as i8) as i16 * 16) as u16)
        };

        let tile_row_address = tile_data_base + (tile_pixel_y as u16) * 2;
        let byte1 = self.ram.fetch(tile_row_address);
        let byte2 = self.ram.fetch(tile_row_address + 1);

        let bit_position = 7 - tile_pixel_x;
        let color_bit_0 = (byte1 >> bit_position) & 1;
        let color_bit_1 = (byte2 >> bit_position) & 1;
        let color_id = (color_bit_1 << 1) | color_bit_0;

        // Get the palette value from BGP register
        let palette = self.get_bg_palette();
        let palette_color = (palette >> (color_id * 2)) & 0x03;

        // Map palette color to grayscale shade (0=white, 3=black)
        let shade = match palette_color {
            0 => 255, // White
            1 => 192, // Light gray
            2 => 96,  // Dark gray
            3 => 0,   // Black
            _ => 0,
        };

        let fb_index = ((self.get_lcd_y() as usize) * 160 + (pixel_x as usize)) * 4;
        self.ppu.framebuffer[fb_index] = shade; // R
        self.ppu.framebuffer[fb_index + 1] = shade; // G
        self.ppu.framebuffer[fb_index + 2] = shade; // B
        self.ppu.framebuffer[fb_index + 3] = 255; // A
    }

    fn set_ppu_mode(&mut self, new_mode: PpuMode) {
        self.ppu.draw_mode = new_mode.clone();
        self.set_lcd_status(self.get_lcd_status() & 0xFC | (new_mode) as u8);
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
