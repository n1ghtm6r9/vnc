use rfb::{ProtocolVersion, SECURITY_NONE};

pub(crate) fn expects_security_result(version: ProtocolVersion, security: u8) -> bool {
    matches!(
        version,
        ProtocolVersion::V3_8 | ProtocolVersion::AppleRemoteDesktop
    ) || security != SECURITY_NONE
}
