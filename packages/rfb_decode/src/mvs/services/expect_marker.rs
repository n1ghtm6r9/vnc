use crate::{DecodeError, MvsBits};

pub(crate) fn expect_marker(
    bits: &mut MvsBits<'_>,
    width: u32,
    marker: u32,
) -> Result<(), DecodeError> {
    let found = bits.read_bits(width)?;
    if found == marker {
        Ok(())
    } else {
        Err(DecodeError::Corrupt(format!(
            "the MVS update ends with {found:#x} where {marker:#x} belongs"
        )))
    }
}
