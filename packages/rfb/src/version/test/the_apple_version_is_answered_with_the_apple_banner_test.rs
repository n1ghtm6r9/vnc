use crate::ProtocolVersion;

#[test]
fn test_the_apple_version_is_answered_with_the_apple_banner() {
    let mut bytes = Vec::new();
    ProtocolVersion::AppleRemoteDesktop.write_client(&mut bytes).unwrap();
    assert_eq!(
        bytes,
        b"RFB 003.889\n".to_vec(),
        "the Mac turns on its Apple Remote Desktop messages only for a client that echoes 3.889"
    );
}
