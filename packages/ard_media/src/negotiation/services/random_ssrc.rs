use super::random_array;
use crate::MediaError;

pub(crate) fn random_ssrc() -> Result<u32, MediaError> {
    loop {
        let ssrc = u32::from_be_bytes(random_array()?);
        if ssrc != 0 {
            return Ok(ssrc);
        }
    }
}
