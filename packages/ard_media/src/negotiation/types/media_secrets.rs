use crate::KEY_BLOCK_LENGTH;

#[derive(Clone)]
pub struct MediaSecrets {
    pub session_id: [u8; 16],
    pub audio_keys: [u8; KEY_BLOCK_LENGTH],
    pub video_keys: [u8; KEY_BLOCK_LENGTH],
    pub audio_ssrc: u32,
    pub video_ssrc: u32,
    pub call_ids: [String; 2],
}
