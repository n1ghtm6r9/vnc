#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PayloadPart {
    Units(Vec<Vec<u8>>),
    Fragment { header: [u8; 3], data: Vec<u8> },
}
