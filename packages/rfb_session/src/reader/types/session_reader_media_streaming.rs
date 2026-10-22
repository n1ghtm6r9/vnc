use super::SessionReader;
use crate::SessionMedia;

impl SessionReader {
    pub(super) fn media_streaming(&self) -> bool {
        self.media.as_ref().is_some_and(SessionMedia::is_streaming)
    }
}
