use crate::{ServerMessage, write_apple_state_change};

#[test]
fn test_an_apple_state_change_round_trips() {
    let mut bytes = Vec::new();
    write_apple_state_change(&mut bytes, 4).unwrap();
    assert_eq!(bytes, vec![0x14, 0, 0, 4, 0, 0, 0, 4], "the type, padding, a body of four bytes, no flags and the status");
    assert_eq!(
        ServerMessage::read(&mut &bytes[..]).unwrap(),
        ServerMessage::AppleStateChange { status: 4 },
        "the client reads the status back"
    );
}
