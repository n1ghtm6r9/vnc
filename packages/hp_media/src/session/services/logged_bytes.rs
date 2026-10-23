use crate::LOGGED_BYTES_LIMIT;

pub(crate) fn logged_bytes(payload: &[u8]) -> &[u8] {
    &payload[..payload.len().min(LOGGED_BYTES_LIMIT)]
}
