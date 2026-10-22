#[derive(Debug, Clone, Copy)]
pub(crate) struct MvsTileDescriptor {
    pub(super) threshold: u8,
    pub(super) luma: [i8; 64],
    pub(super) blue_dc: Option<i8>,
    pub(super) red_dc: Option<i8>,
    pub(super) frame: u32,
    pub(super) reference: Option<usize>,
    pub(super) match_source: (i32, i32),
}

impl Default for MvsTileDescriptor {
    fn default() -> MvsTileDescriptor {
        MvsTileDescriptor {
            threshold: 0,
            luma: [0; 64],
            blue_dc: None,
            red_dc: None,
            frame: 0,
            reference: None,
            match_source: (0, 0),
        }
    }
}
