use super::{MvsBits, MvsState, MvsTileDescriptor};
use crate::{DecodeError, MVS_CACHE_ENTRY_BYTES, MVS_LUMA, read_luma_delta, read_small_delta};

impl MvsState {
    pub(super) fn partial_luma(
        &mut self,
        bits: &mut MvsBits<'_>,
        descriptor: &MvsTileDescriptor,
        entry: &mut [i8; MVS_CACHE_ENTRY_BYTES],
    ) -> Result<(), DecodeError> {
        let threshold = usize::from(descriptor.threshold);
        let fine = threshold >= 15;
        let luma = &mut self.coefficients[MVS_LUMA];
        luma[0] = i16::from(descriptor.luma[0]);
        entry[0] = descriptor.luma[0];
        let end = bits.read_bits(6)? as usize + 1;
        let mut baselines = descriptor
            .luma
            .iter()
            .copied()
            .skip(1)
            .chain(std::iter::repeat(0));
        let mut written = 1;
        for slot in luma.iter_mut().take(end.min(threshold)).skip(1) {
            let baseline = baselines.next().unwrap_or(0);
            let value = if fine {
                read_small_delta(bits, baseline)?
            } else {
                read_luma_delta(bits, baseline, 3)?
            };
            *slot = i16::from(value);
            entry[written] = value;
            written += 1;
        }
        if end > threshold {
            let delta_bits = if fine { 3 } else { 4 };
            for slot in luma.iter_mut().take(end).skip(threshold) {
                let baseline = baselines.next().unwrap_or(0);
                let value = read_luma_delta(bits, baseline, delta_bits)?;
                *slot = i16::from(value);
                entry[written] = value;
                written += 1;
            }
        }
        Ok(())
    }
}
