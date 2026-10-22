use std::{
    io::{ErrorKind, Write},
    net::Shutdown,
};

use rfb::{ENCODING_RAW, Rect, RectHeader, write_framebuffer_update_header};
use rfb_decode::DecodeError;

use crate::test_support::{
    FakeServer, events_until_disconnected, serve_until_connected, start_with_events, test_options,
};
use crate::{SessionError, SessionEvent};

#[test]
fn test_a_server_that_hangs_up_mid_update_ends_with_an_error() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        RectHeader {
            rect: Rect::new(0, 0, 64, 48),
            encoding: ENCODING_RAW,
        }
        .write(stream)
        .expect("the fake server announces a full-screen raw rect");
        stream
            .write_all(&[0x80; 64 * 24 * 4])
            .expect("the fake server sends half of the pixels");
        stream
            .shutdown(Shutdown::Write)
            .expect("the fake server hangs up");
    });
    let (_session, events) = start_with_events(test_options(server.port()));

    let events = events_until_disconnected(&events);
    server.outcome();

    assert!(
        matches!(
            events.as_slice(),
            [
                SessionEvent::Connected { .. },
                SessionEvent::Disconnected {
                    error: Some(SessionError::Decode(DecodeError::Io(cause)))
                }
            ] if cause.kind() == ErrorKind::UnexpectedEof
        ),
        "a server that hangs up inside a rect ends the session with a read error and no frame, got {events:?}"
    );
}
