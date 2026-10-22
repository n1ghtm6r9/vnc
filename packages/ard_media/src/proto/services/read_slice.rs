use crate::MediaError;

pub(crate) fn read_slice<'a>(
    bytes: &'a [u8],
    position: &mut usize,
    length: u64,
) -> Result<&'a [u8], MediaError> {
    let end = usize::try_from(length)
        .ok()
        .and_then(|length| position.checked_add(length))
        .filter(|end| *end <= bytes.len())
        .ok_or(MediaError::Truncated("protocol buffer field"))?;
    let slice = &bytes[*position..end];
    *position = end;
    Ok(slice)
}
