use super::be_u16;
use crate::{MediaError, Message1};

pub fn parse_message1(payload: &[u8]) -> Result<Message1, MediaError> {
    let length = payload
        .get(..2)
        .map(|_| usize::from(be_u16(payload, 0)))
        .filter(|length| *length >= 16 && payload.len() >= 2 + length)
        .ok_or(MediaError::Truncated("High Performance offer (Message 1)"))?;
    let body = &payload[2..2 + length];
    let mut strings = Vec::with_capacity(3);
    let mut offset = 16;
    for size in [be_u16(body, 10), be_u16(body, 12), be_u16(body, 14)] {
        let end = offset + usize::from(size);
        let text = body.get(offset..end).ok_or_else(|| {
            MediaError::Malformed("High Performance offer (Message 1) string length".into())
        })?;
        let trimmed = text
            .iter()
            .rposition(|byte| *byte != 0)
            .map_or(&text[..0], |last| &text[..=last]);
        strings.push(String::from_utf8_lossy(trimmed).into_owned());
        offset = end;
    }
    Ok(Message1 {
        model: strings[0].clone(),
        strings,
    })
}
