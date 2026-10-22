use rfb::{APPLE_STATE_TICKLE, ClientMessage, Rect, write_apple_state_change};

use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_a_tickle_from_the_mac_turns_pushing_on_again() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_apple_state_change(stream, APPLE_STATE_TICKLE).expect("the fake server tickles");
        read_client_messages(stream, 1)
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let received = server.outcome();

    assert_eq!(
        received,
        vec![ClientMessage::AppleAutoUpdate {
            enabled: true,
            rect: Rect::new(0, 0, 64, 48)
        }],
        "a tickle from the Mac is answered by turning pushed updates on again"
    );
}
