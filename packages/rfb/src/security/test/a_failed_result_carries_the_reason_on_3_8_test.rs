use crate::{ProtocolError, ProtocolVersion, read_security_result, write_security_result};

#[test]
fn test_a_failed_result_carries_the_reason_on_3_8() {
    let mut bytes = Vec::new();
    write_security_result(&mut bytes, Some("authentication failed")).unwrap();
    let error = read_security_result(&mut &bytes[..], ProtocolVersion::V3_8).unwrap_err();
    assert!(
        matches!(error, ProtocolError::AuthenticationFailed(ref reason) if reason == "authentication failed"),
        "a 3.8 failure carries the server's reason, got {error:?}"
    );
}
