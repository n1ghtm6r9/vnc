use super::{MvsBits, MvsState};
use crate::{
    DecodeError, MVS_BLUE, MVS_CACHE_ENTRY_BYTES, MVS_CACHED_LUMA, MVS_CACHED_RED, MVS_RED,
    read_huffman_ac, read_small_delta,
};

impl MvsState {
    pub(super) fn partial_dct_tile(
        &mut self,
        bits: &mut MvsBits<'_>,
        screen_index: usize,
        limits: [usize; 2],
    ) -> Result<[u32; 64], DecodeError> {
        let descriptor = self
            .descriptors
            .get(screen_index)
            .copied()
            .unwrap_or_default();
        let (Some(red_baseline), Some(blue_baseline)) = (descriptor.red_dc, descriptor.blue_dc)
        else {
            return Err(DecodeError::Corrupt(
                "an MVS partial update recodes a tile no full update has drawn".to_owned(),
            ));
        };
        self.coefficients = [[0; 64]; 3];
        let mut entry = [0i8; MVS_CACHE_ENTRY_BYTES];
        self.partial_luma(bits, &descriptor, &mut entry)?;
        self.coefficients[MVS_RED][0] = i16::from(read_small_delta(bits, red_baseline)?);
        read_huffman_ac(bits, &mut self.coefficients[MVS_RED], limits[0])?;
        self.coefficients[MVS_BLUE][0] = i16::from(read_small_delta(bits, blue_baseline)?);
        read_huffman_ac(bits, &mut self.coefficients[MVS_BLUE], limits[1])?;
        let red_entry = entry[MVS_CACHED_LUMA..].iter_mut();
        for (slot, &value) in red_entry.zip(&self.coefficients[MVS_RED][..MVS_CACHED_RED]) {
            *slot = value as i8;
        }
        let blue_entry = entry[MVS_CACHED_LUMA + MVS_CACHED_RED..].iter_mut();
        for (slot, &value) in blue_entry.zip(&self.coefficients[MVS_BLUE]) {
            *slot = value as i8;
        }
        let pixels = self.render_tile();
        self.cache.store(&entry);
        Ok(pixels)
    }
}
