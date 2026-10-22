use crate::{ProtocolError, ServerMessage};

#[test]
fn test_an_unknown_server_message_is_refused() {
    let error = ServerMessage::read(&mut &[0x7fu8][..]).unwrap_err();
    assert!(matches!(error, ProtocolError::UnknownMessage(0x7f)), "an unknown message cannot be skipped, got {error:?}");
}
