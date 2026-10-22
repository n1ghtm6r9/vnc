use crate::ProtocolVersion;

#[test]
fn test_the_client_banner_is_twelve_bytes() {
    let mut bytes = Vec::new();
    ProtocolVersion::V3_8.write_client(&mut bytes).unwrap();
    assert_eq!(bytes, b"RFB 003.008\n", "the client answers a 3.889 server with plain 3.8");
}
