use super::Phrase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notice {
    pub phrase: Phrase,
    pub detail: Option<String>,
}
