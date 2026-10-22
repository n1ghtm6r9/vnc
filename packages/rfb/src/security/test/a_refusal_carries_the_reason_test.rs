use crate::{ProtocolError, ProtocolVersion, read_security_types, write_text};

#[test]
fn test_a_refusal_carries_the_reason() {
    let mut bytes = vec![0u8];
    write_text(&mut bytes, b"too many connections").unwrap();
    let error = read_security_types(&mut &bytes[..], ProtocolVersion::V3_8).unwrap_err();
    assert!(
        matches!(error, ProtocolError::ConnectionRefused(ref reason) if reason == "too many connections"),
        "an empty list is a refusal and its reason must reach the user, got {error:?}"
    );
}
