use rfb::ClientMessage;

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_apple_pointer_buttons_swap_middle_and_right() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        read_client_messages(stream, 4)
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    for buttons in [1, 2, 4, 0] {
        session.send(ClientCommand::Pointer {
            x: 5,
            y: 6,
            buttons,
        });
    }
    let received = server.outcome();

    let sent: Vec<u8> = received
        .iter()
        .map(|message| match message {
            ClientMessage::PointerEvent {
                buttons,
                x: 5,
                y: 6,
            } => *buttons,
            other => panic!("a pointer event goes out as one, got {other:?}"),
        })
        .collect();
    assert_eq!(
        sent,
        vec![1, 4, 2, 0],
        "the Mac numbers the middle and right buttons the other way round"
    );
}
