use crate::{Language, Notice, Phrase};

pub struct ReconnectView {
    pub language: Language,
    pub heading: Phrase,
    pub reason: Option<Notice>,
    pub seconds_left: Option<u64>,
}
