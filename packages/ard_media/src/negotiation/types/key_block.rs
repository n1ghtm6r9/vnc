use crate::MasterKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyBlock {
    pub to_server: MasterKey,
    pub from_server: MasterKey,
}
