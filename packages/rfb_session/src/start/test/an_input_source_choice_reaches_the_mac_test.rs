use rfb::ClientMessage;

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_an_input_source_choice_reaches_the_mac() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    session.send(ClientCommand::SetInputSource("com.apple.keylayout.ABC".to_owned()));
    let received = server.outcome();

    assert_eq!(
        received,
        vec![ClientMessage::AppleSetInputSource {
            source: "com.apple.keylayout.ABC".to_owned()
        }],
        "the input source choice reaches the Mac as SetKeyboardInputSource"
    );
}
