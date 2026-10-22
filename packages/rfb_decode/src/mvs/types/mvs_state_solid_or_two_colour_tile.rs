use super::{MvsBits, MvsState};
use crate::{DecodeError, read_two_colour_bitmap, read_ycbcr20};

impl MvsState {
    pub(super) fn solid_or_two_colour_tile(
        &mut self,
        bits: &mut MvsBits<'_>,
    ) -> Result<[u32; 64], DecodeError> {
        let two_colour = bits.read_bit()? == 1;
        let reuse = bits.read_bit()? == 1;
        if !two_colour {
            if !reuse {
                self.solid = read_ycbcr20(bits)?;
            }
            return Ok([self.solid; 64]);
        }
        if !reuse {
            self.set_colour = read_ycbcr20(bits)?;
            self.clear_colour = read_ycbcr20(bits)?;
        }
        read_two_colour_bitmap(bits, self.set_colour, self.clear_colour)
    }
}
