use super::write_apple_field;
use crate::{APPLE_BLOCK_LENGTH, APPLE_FIELD_LENGTH};

pub fn apple_credentials_block(
    username: &str,
    password: &str,
    filler: &[u8; APPLE_BLOCK_LENGTH],
) -> [u8; APPLE_BLOCK_LENGTH] {
    let mut block = *filler;
    let (username_field, password_field) = block.split_at_mut(APPLE_FIELD_LENGTH);
    write_apple_field(username_field, username);
    write_apple_field(password_field, password);
    block
}
