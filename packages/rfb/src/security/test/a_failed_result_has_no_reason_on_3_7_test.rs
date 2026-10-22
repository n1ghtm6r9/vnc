use crate::{ProtocolError, ProtocolVersion, read_security_result};

#[test]
fn test_a_failed_result_has_no_reason_on_3_7() {
    let bytes = [0u8, 0, 0, 1];
    let error = read_security_result(&mut &bytes[..], ProtocolVersion::V3_7).unwrap_err();
    assert!(
        matches!(error, ProtocolError::AuthenticationFailed(ref reason) if reason.is_empty()),
        "a 3.7 failure has no reason text to read, got {error:?}"
    );
}
