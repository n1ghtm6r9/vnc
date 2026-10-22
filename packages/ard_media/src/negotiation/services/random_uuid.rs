use super::random_array;
use crate::MediaError;

pub(crate) fn random_uuid() -> Result<[u8; 16], MediaError> {
    let mut uuid: [u8; 16] = random_array()?;
    uuid[6] = (uuid[6] & 0x0f) | 0x40;
    uuid[8] = (uuid[8] & 0x3f) | 0x80;
    Ok(uuid)
}
