use crate::{DecodeError, MvsBits};

pub(crate) fn read_skip_run(bits: &mut MvsBits<'_>, position: usize) -> Result<usize, DecodeError> {
    let extension = bits.read_bits(2)? as usize;
    if extension < 3 {
        return Ok(extension + 3);
    }
    let mut run = 6;
    loop {
        let chunk = bits.read_bits(3)? as usize;
        run += chunk;
        if chunk < 7 || run + position >= 64 {
            return Ok(run);
        }
    }
}
