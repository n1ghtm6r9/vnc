use super::{random_array, random_ssrc, random_uuid, uuid_string};
use crate::{MediaError, MediaSecrets};

pub fn generate_secrets() -> Result<MediaSecrets, MediaError> {
    Ok(MediaSecrets {
        session_id: random_uuid()?,
        audio_keys: random_array()?,
        video_keys: random_array()?,
        audio_ssrc: random_ssrc()?,
        video_ssrc: random_ssrc()?,
        call_ids: [uuid_string(&random_uuid()?), uuid_string(&random_uuid()?)],
    })
}
