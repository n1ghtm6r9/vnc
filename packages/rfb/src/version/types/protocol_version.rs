#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolVersion {
    V3_3,
    V3_7,
    V3_8,
    AppleRemoteDesktop,
}
