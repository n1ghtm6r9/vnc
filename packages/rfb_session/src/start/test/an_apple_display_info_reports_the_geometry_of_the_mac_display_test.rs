use std::io::Write;

use rfb::{AppleDisplayGeometry, ENCODING_APPLE_DISPLAY_INFO_2, Rect, RectHeader, write_framebuffer_update_header};

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_an_apple_display_info_reports_the_geometry_of_the_mac_display() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        RectHeader {
            rect: Rect::new(0, 0, 0, 0),
            encoding: ENCODING_APPLE_DISPLAY_INFO_2,
        }
        .write(stream)
        .expect("the fake server announces its display");
        let mut payload = vec![0, 20, 0, 5, 0, 40, 0, 30, 0, 80, 0, 60];
        payload.extend_from_slice(&[0; 10]);
        stream
            .write_all(&payload)
            .expect("the fake server describes its display");
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
        "a Mac display of 40x30 points drawn with 80x60 pixels reaches the app, got {described:?}"
    );
}
