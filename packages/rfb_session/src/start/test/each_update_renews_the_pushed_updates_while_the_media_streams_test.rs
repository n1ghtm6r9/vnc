use std::{io::Write, sync::mpsc};

use hp_media::{MediaEvent, MediaState};
use rfb::{
    ClientMessage, ENCODING_APPLE_CURSOR_ALPHA, Rect, RectHeader, write_bell,
    write_framebuffer_update_header,
};

use crate::test_support::{
    EVENT_TIMEOUT, FakeServer, fake_media_starter, high_performance_test_options, next_event,
    next_media_start, read_client_messages, serve_high_performance_until_connected,
    start_with_media,
};
use crate::{ClientCommand, SessionEvent};

#[test]
fn test_each_update_renews_the_pushed_updates_while_the_media_streams() {
    let (go, wait) = mpsc::channel();
    let server = FakeServer::start(move |stream| {
        serve_high_performance_until_connected(stream, 64, 48);
        wait.recv_timeout(EVENT_TIMEOUT)
            .expect("the test lets the fake server go on");
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        RectHeader {
            rect: Rect::new(0, 0, 16, 16),
            encoding: ENCODING_APPLE_CURSOR_ALPHA,
        }
        .write(stream)
        .expect("the fake server announces a cursor");
        stream
            .write_all(&[0, 0, 0, 5, 0, 0, 0, 0])
            .expect("the fake server names the pointing hand");
        write_bell(stream).expect("the fake server rings the bell");
        read_client_messages(stream, 1)
    });
    let (start_media, starts) = fake_media_starter();
    let (session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let mut media = next_media_start(&starts);
    (media.emit)(MediaEvent::State(MediaState::Streaming {
        width: 64,
        height: 48,
        stripes: 1,
    }));
    next_event(&events);
    go.send(()).expect("the fake server waits for the test");
    while !matches!(next_event(&events), SessionEvent::Bell) {}
    session.send(ClientCommand::Pointer {
        x: 1,
        y: 2,
        buttons: 0,
    });
    let received = server.outcome();
    assert_eq!(
        received,
        vec![ClientMessage::AppleAutoUpdate {
            enabled: true,
            rect: Rect::new(0, 0, 64, 48)
        }],
        "while the media streams, each update from the Mac renews the pushed updates, so cursor changes keep coming"
    );
}
