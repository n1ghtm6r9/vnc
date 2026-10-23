use rfb::write_framebuffer_update_header;

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, apple_test_options, drain_until_closed, next_event, serve_apple_until_connected,
    start_with_events, write_input_source_rect,
};

#[test]
fn test_the_mac_input_source_reaches_the_app() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        write_input_source_rect(stream, "com.apple.keylayout.Russian");
        drain_until_closed(stream)
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let reported = next_event(&events);
    drop(session);
    server.outcome();

    assert!(
        matches!(&reported, SessionEvent::InputSource(source) if source == "com.apple.keylayout.Russian"),
        "the input source the Mac reports reaches the app, got {reported:?}"
    );
}
