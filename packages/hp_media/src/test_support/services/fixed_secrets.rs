use ard_media::{MediaError, MediaSecrets};

pub fn fixed_secrets() -> Result<MediaSecrets, MediaError> {
    Ok(MediaSecrets {
        session_id: std::array::from_fn(|index| 0xa0 + index as u8),
        audio_keys: std::array::from_fn(|index| index as u8),
        video_keys: std::array::from_fn(|index| 0xff - index as u8),
        audio_ssrc: 0x1122_3344,
        video_ssrc: 0xf1e2_d3c4,
        call_ids: [
            "11111111-2222-4333-8444-555555555555".into(),
            "AAAAAAAA-BBBB-4CCC-8DDD-EEEEEEEEEEEE".into(),
        ],
    })
}
