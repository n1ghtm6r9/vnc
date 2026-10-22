use super::MvsBits;

pub(crate) struct MvsFullPass<'a> {
    pub(super) body: MvsBits<'a>,
    pub(super) thresholds: [u8; 2],
    pub(super) previous_dct: Option<usize>,
    pub(super) previous_tile: (i32, i32),
}
