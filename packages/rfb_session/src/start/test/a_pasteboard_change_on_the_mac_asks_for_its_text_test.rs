use rfb::{
    APPLE_STATE_PASTEBOARD_CHANGED, ClientMessage, write_apple_clipboard, write_apple_state_change,
};

use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_a_pasteboard_change_on_the_mac_asks_for_its_text() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_apple_clipboard(stream, 9, "old").expect("the fake server sends its pasteboard");
        write_apple_state_change(stream, APPLE_STATE_PASTEBOARD_CHANGED)
            .expect("the fake server reports a new pasteboard");
        read_client_messages(stream, 1)
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let received = server.outcome();

    assert_eq!(
        received,
        vec![ClientMessage::AppleClipboardRequest { session_id: 9 }],
        "a changed pasteboard on the Mac is asked for under the session the Mac named"
    );
}
