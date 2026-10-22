use super::{be_u16, read_media_blob, required_varint};
use crate::{MediaAnswer, MediaError, VideoFormat, decode_proto, proto_bytes};

const MAX_SIDE: u64 = 16384;

pub fn parse_answer(payload: &[u8]) -> Result<MediaAnswer, MediaError> {
    let length = payload
        .get(..2)
        .map(|_| usize::from(be_u16(payload, 0)))
        .filter(|length| *length >= 18 && payload.len() >= 2 + length)
        .ok_or(MediaError::Truncated("media answer"))?;
    let body = &payload[2..2 + length];
    let audio_size = usize::from(be_u16(body, 8));
    let video_size = usize::from(be_u16(body, 10));
    if audio_size == 0 || video_size == 0 || 18 + audio_size + video_size > body.len() {
        return Err(MediaError::Malformed("media answer stream sizes".into()));
    }
    let audio_blob = read_media_blob(&body[18..18 + audio_size])?;
    let video_blob = read_media_blob(&body[18 + audio_size..18 + audio_size + video_size])?;
    let audio =
        decode_proto(proto_bytes(&audio_blob, 3).ok_or_else(|| {
            MediaError::Malformed("media answer without audio parameters".into())
        })?)?;
    let video =
        decode_proto(proto_bytes(&video_blob, 5).ok_or_else(|| {
            MediaError::Malformed("media answer without video parameters".into())
        })?)?;
    let width = required_varint(&video, 4, "picture width")?;
    let height = required_varint(&video, 5, "picture height")?;
    let stripes = required_varint(&video, 6, "stripe count")?;
    if !(1..=MAX_SIDE).contains(&width)
        || !(1..=MAX_SIDE).contains(&height)
        || !(1..=height).contains(&stripes)
    {
        return Err(MediaError::Malformed(format!(
            "media answer picture of {width}x{height} in {stripes} stripes"
        )));
    }
    Ok(MediaAnswer {
        version: be_u16(body, 0),
        audio_ssrc: required_varint(&audio, 1, "audio source")? as u32,
        video: VideoFormat {
            ssrc: required_varint(&video, 1, "video source")? as u32,
            width: width as u32,
            height: height as u32,
            stripes: stripes as u32,
        },
    })
}
