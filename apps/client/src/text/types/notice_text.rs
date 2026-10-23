use super::Notice;
use crate::Language;

impl Notice {
    pub fn text(&self, language: Language) -> String {
        let phrase = self.phrase.text(language);
        match &self.detail {
            Some(detail) => format!("{phrase} {detail}"),
            None => phrase.to_owned(),
        }
    }
}
