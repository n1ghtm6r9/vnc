use rfb::{ClientMessage, Rect};

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_refresh_in_apple_mode_turns_pushing_on_again() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        read_client_messages(stream, 2)
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    session.send(ClientCommand::RefreshScreen);
    let received = server.outcome();

    let screen = Rect::new(0, 0, 64, 48);
    assert_eq!(
        received,
        vec![
            ClientMessage::FramebufferUpdateRequest {
                incremental: false,
                rect: screen
            },
            ClientMessage::AppleAutoUpdate {
                enabled: true,
                rect: screen
            },
        ],
        "a refresh asks for the whole screen and turns pushed updates on again"
    );
}
