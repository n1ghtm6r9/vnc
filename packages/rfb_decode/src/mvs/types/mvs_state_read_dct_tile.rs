use super::{MvsFullPass, MvsState};
use crate::{
    DecodeError, MVS_BLUE, MVS_LUMA, MVS_RED, MvsAcSymbol, halve_toward_zero, read_ac_symbol,
    read_dc_rice,
};

impl MvsState {
    pub(super) fn read_dct_tile(
        &mut self,
        pass: &mut MvsFullPass<'_>,
        screen_index: usize,
    ) -> Result<(), DecodeError> {
        let bits = &mut pass.body;
        let chroma_coded = bits.read_bit()? == 0;
        let threshold = pass.thresholds[bits.read_bit()? as usize];
        for plane in [MVS_BLUE, MVS_RED] {
            self.coefficients[plane][0] = if chroma_coded {
                let rice = read_dc_rice(bits)?;
                halve_toward_zero(self.previous[plane][0])
                    .wrapping_sub(rice)
                    .wrapping_mul(2) as i16
            } else {
                self.previous[plane][0]
            };
        }
        let luma_rice = read_dc_rice(bits)?;
        self.coefficients[MVS_LUMA][0] =
            i32::from(self.previous[MVS_LUMA][0]).wrapping_sub(luma_rice) as i16;
        let mut position = 1;
        while position < 64 {
            match read_ac_symbol(bits, position, threshold)? {
                MvsAcSymbol::EndOfBlock => break,
                MvsAcSymbol::Skip(run) => position += run,
                MvsAcSymbol::Value(value) => {
                    self.coefficients[MVS_LUMA][position] = value as i16;
                    position += 1;
                }
            }
        }
        if let Some(descriptor) = self.descriptors.get_mut(screen_index) {
            descriptor.threshold = threshold;
            descriptor.luma = self.coefficients[MVS_LUMA].map(|value| value as i8);
            descriptor.blue_dc = Some(self.coefficients[MVS_BLUE][0] as i8);
            descriptor.red_dc = Some(self.coefficients[MVS_RED][0] as i8);
        }
        Ok(())
    }
}
