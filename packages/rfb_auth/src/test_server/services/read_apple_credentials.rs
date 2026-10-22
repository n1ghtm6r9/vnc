use super::read_apple_field;
use crate::{APPLE_BLOCK_LENGTH, APPLE_FIELD_LENGTH, AuthError, Credentials};

pub(crate) fn read_apple_credentials(
    block: &[u8; APPLE_BLOCK_LENGTH],
) -> Result<Credentials, AuthError> {
    let (username_field, password_field) = block.split_at(APPLE_FIELD_LENGTH);
    Ok(Credentials::new(
        read_apple_field(username_field)?,
        read_apple_field(password_field)?,
    ))
}
