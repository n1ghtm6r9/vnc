use crate::AuthError;

pub(crate) fn fill_random(buffer: &mut [u8]) -> Result<(), AuthError> {
    getrandom::fill(buffer).map_err(|failure| AuthError::Random(failure.to_string()))
}
