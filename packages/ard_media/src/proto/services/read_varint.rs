use crate::MediaError;

pub(crate) fn read_varint(bytes: &[u8], position: &mut usize) -> Result<u64, MediaError> {
    let mut value = 0u64;
    for index in 0..10 {
        let byte = *bytes
            .get(*position)
            .ok_or(MediaError::Truncated("protocol buffer varint"))?;
        *position += 1;
        if index == 9 && byte > 1 {
            return Err(MediaError::Malformed(
                "protocol buffer varint over 64 bits".into(),
            ));
        }
        value |= u64::from(byte & 0x7f) << (7 * index);
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(MediaError::Malformed(
        "protocol buffer varint over 10 bytes".into(),
    ))
}
