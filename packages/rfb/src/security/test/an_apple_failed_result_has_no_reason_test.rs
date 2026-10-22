use crate::{ProtocolError, ProtocolVersion, read_security_result};

#[test]
fn test_an_apple_failed_result_has_no_reason() {
    let bytes = [0u8, 0, 0, 1, 0, 0, 0, 3, b'b', b'a', b'd'];
    let mut reader = &bytes[..];
    let error = read_security_result(&mut reader, ProtocolVersion::AppleRemoteDesktop).unwrap_err();
    assert!(
        matches!(error, ProtocolError::AuthenticationFailed(ref reason) if reason.is_empty()),
        "the Mac sends no reason after a failed Apple Remote Desktop login, got {error:?}"
    );
    assert_eq!(reader.len(), 7, "nothing after the status is read as a reason");
}
