use super::{audio_blob_fields, stream_offer, video_blob_fields};
use crate::{
    KEY_BLOCK_LENGTH, MODE_AUDIO, MODE_VIDEO, MSG_MEDIA_STREAM_NEGOTIATION, MediaError,
    MediaSecrets, OFFER_FLAGS, OFFER_HEADER_LENGTH, OFFER_VERSION,
};

pub fn build_offer(secrets: &MediaSecrets, ntp_time: u64) -> Result<Vec<u8>, MediaError> {
    let audio = stream_offer(
        MODE_AUDIO,
        &audio_blob_fields(secrets.audio_ssrc, ntp_time),
        &secrets.call_ids[0],
    );
    let video = stream_offer(
        MODE_VIDEO,
        &video_blob_fields(secrets.video_ssrc, ntp_time),
        &secrets.call_ids[1],
    );
    let total = OFFER_HEADER_LENGTH + 2 * KEY_BLOCK_LENGTH + audio.len() + video.len();
    let (Ok(length), Ok(audio_length), Ok(video_length)) = (
        u16::try_from(total - 4),
        u16::try_from(audio.len()),
        u16::try_from(video.len()),
    ) else {
        return Err(MediaError::OfferTooLarge(total));
    };
    let mut message = Vec::with_capacity(total);
    message.extend_from_slice(&[MSG_MEDIA_STREAM_NEGOTIATION, 0]);
    message.extend_from_slice(&length.to_be_bytes());
    message.extend_from_slice(&OFFER_VERSION.to_be_bytes());
    message.extend_from_slice(&OFFER_FLAGS.to_be_bytes());
    message.extend_from_slice(&audio_length.to_be_bytes());
    message.extend_from_slice(&video_length.to_be_bytes());
    message.resize(20, 0);
    message.extend_from_slice(&secrets.session_id);
    message.extend_from_slice(&secrets.audio_keys);
    message.extend_from_slice(&audio);
    message.extend_from_slice(&secrets.video_keys);
    message.extend_from_slice(&video);
    Ok(message)
}
