use crate::{ClientMessage, ProtocolError};

#[test]
fn test_an_unknown_client_message_is_refused() {
    let error = ClientMessage::read(&mut &[250u8][..]).unwrap_err();
    assert!(matches!(error, ProtocolError::UnknownMessage(250)), "a type nobody defined cannot be skipped, got {error:?}");
}
