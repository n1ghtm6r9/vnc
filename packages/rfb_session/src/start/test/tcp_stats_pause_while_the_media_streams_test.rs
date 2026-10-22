use std::{sync::mpsc, thread, time::Duration};

use hp_media::{MediaEvent, MediaState};
use rfb::{Rect, write_bell, write_framebuffer_update_header};

use crate::SessionEvent;
use crate::test_support::{
    EVENT_TIMEOUT, FakeServer, fake_media_starter, high_performance_test_options, next_event,
    next_media_start, serve_high_performance_until_connected, start_with_media, write_raw_rect,
};

#[test]
fn test_tcp_stats_pause_while_the_media_streams() {
    let (go, wait) = mpsc::channel();
    let server = FakeServer::start(move |stream| {
        serve_high_performance_until_connected(stream, 64, 48);
        wait.recv_timeout(EVENT_TIMEOUT)
            .expect("the test lets the fake server go on");
        write_framebuffer_update_header(stream, 1)
            .expect("the fake server starts the first update");
        write_raw_rect(stream, Rect::new(0, 0, 64, 48), 0xff00_00ff);
        thread::sleep(Duration::from_millis(1_100));
        write_framebuffer_update_header(stream, 1)
            .expect("the fake server starts the second update");
        write_raw_rect(stream, Rect::new(0, 0, 8, 8), 0xff00_ff00);
        write_bell(stream).expect("the fake server rings the bell");
    });
    let (start_media, starts) = fake_media_starter();
    let (_session, events) =
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
    let after = next_event(&events);

    assert!(
        matches!(after, SessionEvent::Bell),
        "while the media streams, the TCP updates bring no stats of their own, got {after:?}"
    );
}
