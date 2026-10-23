use std::io::Write;

use rfb::{SERVER_BELL, write_framebuffer_update_header};

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, serve_apple_until_connected, start_with_events,
    write_input_source_rect,
};

#[test]
fn test_a_repeated_input_source_reaches_the_app_once() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 3).expect("the fake server starts an update");
        for source in [
            "com.apple.keylayout.Russian",
            "com.apple.keylayout.Russian",
            "com.apple.keylayout.ABC",
        ] {
            write_input_source_rect(stream, source);
        }
        stream
            .write_all(&[SERVER_BELL])
            .expect("the fake server rings the bell");
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let mut reported = Vec::new();
    loop {
        match next_event(&events) {
            SessionEvent::Bell => break,
            SessionEvent::InputSource(source) => reported.push(source),
            _ => {}
        }
    }
    server.outcome();

    assert_eq!(
        reported,
        ["com.apple.keylayout.Russian", "com.apple.keylayout.ABC"],
        "the Mac repeats its input source, and only the changes reach the app"
    );
}
