use rfb::{ClientMessage, Rect, write_framebuffer_update_header};

use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events, write_raw_rect,
};
use crate::{ClientCommand, SessionEvent};

#[test]
fn test_apple_updates_arrive_without_requests() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        write_raw_rect(stream, Rect::new(0, 0, 64, 48), 0xff00_00ff);
        read_client_messages(stream, 1)
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let frame = next_event(&events);
    session.send(ClientCommand::Pointer {
        x: 1,
        y: 2,
        buttons: 0,
    });
    let received = server.outcome();

    assert!(
        matches!(frame, SessionEvent::FrameUpdated { .. }),
        "the pushed frame arrives, got {frame:?}"
    );
    assert_eq!(
        received,
        vec![ClientMessage::PointerEvent {
            buttons: 0,
            x: 1,
            y: 2
        }],
        "the Mac pushes updates by itself, so no update request goes out before the next input"
    );
}
