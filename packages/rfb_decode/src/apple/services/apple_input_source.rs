use rfb::{read_bytes, read_u16, read_u32};

pub(crate) fn apple_input_source(mut payload: &[u8]) -> Option<String> {
    let _source_count = read_u16(&mut payload).ok()?;
    let _flags = read_u32(&mut payload).ok()?;
    let name_length = usize::from(read_u16(&mut payload).ok()?);
    let name = read_bytes(&mut payload, name_length).ok()?;
    String::from_utf8(name).ok()
}
