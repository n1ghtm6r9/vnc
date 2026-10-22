use super::SessionReader;
use crate::SessionMedia;

impl SessionReader {
    pub(crate) fn with_media(mut self, media: Option<SessionMedia>) -> SessionReader {
        self.media = media;
        self
    }
}
