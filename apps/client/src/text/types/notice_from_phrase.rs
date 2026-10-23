use super::{Notice, Phrase};

impl From<Phrase> for Notice {
    fn from(phrase: Phrase) -> Notice {
        Notice { phrase, detail: None }
    }
}
