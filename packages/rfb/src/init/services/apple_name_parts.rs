use crate::APPLE_NAME_HEADER_LENGTH;

pub(crate) fn apple_name_parts(field: &[u8]) -> (u32, [u8; 16], String) {
    if field.len() < APPLE_NAME_HEADER_LENGTH || field[0] != 0 {
        return (0, [0; 16], String::from_utf8_lossy(field).into_owned());
    }
    let flags = u32::from_be_bytes([field[2], field[3], field[4], field[5]]);
    let mut capabilities = [0u8; 16];
    capabilities.copy_from_slice(&field[6..APPLE_NAME_HEADER_LENGTH]);
    let tail = &field[APPLE_NAME_HEADER_LENGTH..];
    let start = tail.iter().rposition(|&byte| byte == 0).map_or(0, |at| at + 1);
    (flags, capabilities, String::from_utf8_lossy(&tail[start..]).into_owned())
}
