use hp_media::MediaStartError;

use crate::StartMedia;

pub fn failing_media_starter() -> StartMedia {
    Box::new(|_, _| Err(MediaStartError::Stopped))
}
