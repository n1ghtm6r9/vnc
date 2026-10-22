use super::SrtpKeys;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionKeys {
    pub rtp: SrtpKeys,
    pub rtcp: SrtpKeys,
}
