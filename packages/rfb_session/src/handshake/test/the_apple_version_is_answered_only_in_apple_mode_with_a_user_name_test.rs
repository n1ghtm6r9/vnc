use rfb::ProtocolVersion;
use rfb_auth::Credentials;

use crate::test_support::{apple_test_options, test_options};
use crate::{ConnectOptions, choose_version};

#[test]
fn test_the_apple_version_is_answered_only_in_apple_mode_with_a_user_name() {
    let apple = apple_test_options(5900);
    let plain = test_options(5900);
    let nameless = ConnectOptions {
        credentials: Credentials::new("", "password"),
        ..apple_test_options(5900)
    };
    let cases = [
        (
            "Apple mode",
            &apple,
            ProtocolVersion::AppleRemoteDesktop,
            ProtocolVersion::AppleRemoteDesktop,
        ),
        (
            "plain mode",
            &plain,
            ProtocolVersion::AppleRemoteDesktop,
            ProtocolVersion::V3_8,
        ),
        (
            "Apple mode without a user name",
            &nameless,
            ProtocolVersion::AppleRemoteDesktop,
            ProtocolVersion::V3_8,
        ),
        (
            "Apple mode",
            &apple,
            ProtocolVersion::V3_8,
            ProtocolVersion::V3_8,
        ),
        (
            "Apple mode",
            &apple,
            ProtocolVersion::V3_3,
            ProtocolVersion::V3_3,
        ),
    ];
    for (mode, options, offered, expected) in cases {
        assert_eq!(
            choose_version(offered, options),
            expected,
            "in {mode} a server speaking {offered:?} is answered with {expected:?}"
        );
    }
}
