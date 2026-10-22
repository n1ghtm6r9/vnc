use rfb::{APPLE_STATE_PASTEBOARD_DATA_NEEDED, write_apple_state_change};

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_the_mac_asking_for_the_pasteboard_gets_the_last_text() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        let first = read_client_messages(stream, 1);
        write_apple_state_change(stream, APPLE_STATE_PASTEBOARD_DATA_NEEDED)
            .expect("the fake server asks for the pasteboard");
        (first, read_client_messages(stream, 1))
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    session.send(ClientCommand::ClipboardText("текст".to_owned()));
    let (first, again) = server.outcome();

    assert_eq!(
        again, first,
        "the Mac asking for the pasteboard gets the last text sent once more"
    );
}
