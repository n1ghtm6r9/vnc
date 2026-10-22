use std::io;

use crate::AuthError;

pub(crate) fn read_apple_field(field: &[u8]) -> Result<String, AuthError> {
    let end = field
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(field.len());
    String::from_utf8(field[..end].to_vec())
        .map_err(|error| AuthError::Io(io::Error::new(io::ErrorKind::InvalidData, error)))
}
