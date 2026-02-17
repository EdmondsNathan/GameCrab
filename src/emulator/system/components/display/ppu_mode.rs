#[derive(Default, Clone)]
#[repr(u8)]
pub(crate) enum PpuMode {
    #[default]
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Draw = 3,
}
