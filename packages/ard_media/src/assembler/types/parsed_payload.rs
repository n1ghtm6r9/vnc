use super::PayloadPart;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedPayload {
    pub(crate) don: u16,
    pub(crate) part: PayloadPart,
}
