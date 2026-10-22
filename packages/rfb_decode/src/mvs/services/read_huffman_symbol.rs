use crate::{DecodeError, MVS_HUFFMAN_LENGTHS, MVS_HUFFMAN_SYMBOLS, MvsBits};

pub(crate) fn read_huffman_symbol(bits: &mut MvsBits<'_>) -> Result<u8, DecodeError> {
    let mut code = 0u32;
    let mut first = 0u32;
    let mut index = 0usize;
    for &count in &MVS_HUFFMAN_LENGTHS {
        code |= bits.read_bit()?;
        let count = u32::from(count);
        if let Some(offset) = code.checked_sub(first).filter(|&offset| offset < count) {
            return Ok(MVS_HUFFMAN_SYMBOLS[index + offset as usize]);
        }
        index += count as usize;
        first = (first + count) << 1;
        code <<= 1;
    }
    Err(DecodeError::Corrupt(
        "an MVS chroma code is longer than 16 bits".to_owned(),
    ))
}
