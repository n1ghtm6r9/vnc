use crate::ProtocolVersion;

#[test]
fn test_the_apple_banner_is_read_as_the_apple_version() {
    let version = ProtocolVersion::read_server(&mut &b"RFB 003.889\n"[..]).unwrap();
    assert_eq!(
        version,
        ProtocolVersion::AppleRemoteDesktop,
        "macOS Screen Sharing answers 3.889, the Apple Remote Desktop flavour of 3.8"
    );
}
