#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phrase {
    pub english: &'static str,
    pub russian: &'static str,
    pub ukrainian: &'static str,
}
