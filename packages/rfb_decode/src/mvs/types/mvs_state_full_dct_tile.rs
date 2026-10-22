use super::{MvsFullPass, MvsState};
use crate::DecodeError;

impl MvsState {
    pub(super) fn full_dct_tile(
        &mut self,
        pass: &mut MvsFullPass<'_>,
        screen_index: usize,
    ) -> Result<[u32; 64], DecodeError> {
        self.coefficients = [[0; 64]; 3];
        if pass.body.read_bit()? == 1 {
            self.repeat_dct_tile(pass.previous_dct, screen_index);
        } else {
            self.read_dct_tile(pass, screen_index)?;
        }
        self.previous = self.coefficients;
        Ok(self.render_tile())
    }
}
