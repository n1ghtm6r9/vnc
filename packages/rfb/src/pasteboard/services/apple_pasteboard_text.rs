use super::{pasteboard_field, pasteboard_number};
use crate::APPLE_PASTEBOARD_TEXT_TYPE;

pub(crate) fn apple_pasteboard_text(pasteboard: &[u8]) -> Option<String> {
    let mut rest = pasteboard;
    let type_count = pasteboard_number(&mut rest)?;
    for _ in 0..type_count {
        let name = pasteboard_field(&mut rest)?;
        pasteboard_number(&mut rest)?;
        let property_count = pasteboard_number(&mut rest)?;
        for _ in 0..property_count {
            pasteboard_field(&mut rest)?;
            pasteboard_field(&mut rest)?;
        }
        let data = pasteboard_field(&mut rest)?;
        if name == APPLE_PASTEBOARD_TEXT_TYPE.as_bytes() {
            return Some(String::from_utf8_lossy(data).into_owned());
        }
    }
    None
}
