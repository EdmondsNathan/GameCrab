#[derive(Default)]
pub(crate) struct Ppu {
    scanline: u8,
    ppu_mode: u8,
    dots: u16,

    lcd_control: u8,
    lcd_status: u8,
    scroll_y: u8,
    scroll_x: u8,
    ly_compare: u8,
}

impl Ppu {
    pub(crate) fn new() -> Ppu {
        Self::default()
    }

    fn tick(&mut self) {
        todo!();
    }
}
