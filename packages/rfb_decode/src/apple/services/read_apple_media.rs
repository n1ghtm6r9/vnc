use std::io::Read;

use rfb::{read_bytes, read_u16};

use crate::{DecodeError, RectOutcome};

pub fn read_apple_media(reader: &mut impl Read, encoding: i32) -> Result<RectOutcome, DecodeError> {
    let length = read_u16(reader)?;
    let mut payload = length.to_be_bytes().to_vec();
    payload.extend(read_bytes(reader, usize::from(length))?);
    Ok(RectOutcome::AppleMedia { encoding, payload })
}
