use rfb::ClientMessage;

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_apple_keys_travel_encrypted_with_the_session_key() {
    let server = FakeServer::start(|stream| {
        let (login, _) = serve_apple_until_connected(stream, 64, 48);
        (login, read_client_messages(stream, 2))
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    session.send(ClientCommand::Key {
        keysym: 0xff0d,
        down: true,
    });
    session.send(ClientCommand::Key {
        keysym: 0xff0d,
        down: false,
    });
    let (login, received) = server.outcome();

    for (message, down) in received.iter().zip([1, 0]) {
        let ClientMessage::AppleEncryptedEvent { block } = message else {
            panic!("a key goes out as an encrypted event, got {message:?}");
        };
        let plain = login.key.decrypt(block);
        assert_eq!(
            plain[..6],
            [0xff, down, 0, 0, 0xff, 0x0d],
            "the block holds the marker, the press state and the keysym"
        );
        assert_eq!(
            plain[10..],
            [0; 6],
            "the rest of the block after the timestamp is zero"
        );
    }
}
