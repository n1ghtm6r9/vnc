use std::io::Write;

use rfb::AppleDisplayGeometry;

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_a_display_change_on_the_mac_reports_its_geometry() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        let mut message = vec![0x51, 0, 20, 0, 1, 0, 40, 0, 30, 0, 80, 0, 60];
        message.extend_from_slice(&[0; 10]);
        stream
            .write_all(&message)
            .expect("the fake server reports a display change");
        read_client_messages(stream, 2)
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let described = next_event(&events);
    server.outcome();

    assert!(
        matches!(
            described,
            SessionEvent::DisplayGeometry(AppleDisplayGeometry {
                pixel_width: 80,
                pixel_height: 60,
                point_width: 40,
                point_height: 30
            })
        ),
        "a display change of 40x30 points drawn with 80x60 pixels reaches the app, got {described:?}"
    );
}
