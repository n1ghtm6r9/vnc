use super::Phrase;
use crate::Language;

impl Phrase {
    pub fn text(self, language: Language) -> &'static str {
        match language {
            Language::English => self.english,
            Language::Russian => self.russian,
            Language::Ukrainian => self.ukrainian,
        }
    }
}
