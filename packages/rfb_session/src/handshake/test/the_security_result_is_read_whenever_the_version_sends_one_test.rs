use rfb::{ProtocolVersion, SECURITY_APPLE_REMOTE_DESKTOP, SECURITY_NONE, SECURITY_VNC_AUTH};

use crate::expects_security_result;

#[test]
fn test_the_security_result_is_read_whenever_the_version_sends_one() {
    let cases = [
        (ProtocolVersion::AppleRemoteDesktop, SECURITY_NONE, true),
        (
            ProtocolVersion::AppleRemoteDesktop,
            SECURITY_APPLE_REMOTE_DESKTOP,
            true,
        ),
        (ProtocolVersion::V3_8, SECURITY_NONE, true),
        (ProtocolVersion::V3_8, SECURITY_APPLE_REMOTE_DESKTOP, true),
        (ProtocolVersion::V3_7, SECURITY_NONE, false),
        (ProtocolVersion::V3_7, SECURITY_VNC_AUTH, true),
        (ProtocolVersion::V3_3, SECURITY_NONE, false),
        (ProtocolVersion::V3_3, SECURITY_APPLE_REMOTE_DESKTOP, true),
    ];
    for (version, security, expected) in cases {
        assert_eq!(
            expects_security_result(version, security),
            expected,
            "{version:?} after security type {security} sends a security result: {expected}"
        );
    }
}
