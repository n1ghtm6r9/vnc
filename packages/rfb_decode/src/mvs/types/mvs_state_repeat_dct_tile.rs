use super::MvsState;
use crate::{MVS_BLUE, MVS_LUMA, MVS_RED};

impl MvsState {
    pub(super) fn repeat_dct_tile(&mut self, previous_dct: Option<usize>, screen_index: usize) {
        let Some(source) = previous_dct else {
            log::warn!("[DECODE] an MVS tile repeats a DCT tile, but none came before it");
            return;
        };
        self.coefficients[MVS_LUMA] = self.previous[MVS_LUMA];
        self.coefficients[MVS_BLUE][0] = self.previous[MVS_BLUE][0];
        self.coefficients[MVS_RED][0] = self.previous[MVS_RED][0];
        let inherited = self.descriptors.get(source).copied();
        if let (Some(descriptor), Some(target)) =
            (inherited, self.descriptors.get_mut(screen_index))
        {
            *target = descriptor;
        }
    }
}
