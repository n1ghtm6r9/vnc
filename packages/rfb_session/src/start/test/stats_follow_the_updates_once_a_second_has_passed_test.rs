use std::{thread, time::Duration};

use rfb::{ENCODING_RAW, Rect, write_framebuffer_update_header};

use crate::test_support::{
    FakeServer, next_event, serve_until_connected, start_with_events, test_options, write_raw_rect,
};
use crate::{SessionEvent, SessionStats};

#[test]
fn test_stats_follow_the_updates_once_a_second_has_passed() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1)
            .expect("the fake server starts the first update");
        write_raw_rect(stream, Rect::new(0, 0, 64, 48), 0xFF20_4060);
        thread::sleep(Duration::from_millis(1_100));
        write_framebuffer_update_header(stream, 1)
            .expect("the fake server starts the second update");
        write_raw_rect(stream, Rect::new(0, 0, 8, 8), 0xFFE0_C0A0);
    });
    let (_session, events) = start_with_events(test_options(server.port()));

    let connected = next_event(&events);
    let first = next_event(&events);
    let second = next_event(&events);
    let stats = next_event(&events);
    server.outcome();

    assert!(
        matches!(connected, SessionEvent::Connected { .. }),
        "the session connects first, got {connected:?}"
    );
    assert!(
        matches!(first, SessionEvent::FrameUpdated { .. })
            && matches!(second, SessionEvent::FrameUpdated { .. }),
        "both updates arrive as frames, got {first:?} and {second:?}"
    );
    assert!(
        matches!(
            stats,
            SessionEvent::Stats(SessionStats {
                bytes_per_second,
                updates_per_second,
                last_encoding: ENCODING_RAW,
            }) if bytes_per_second > 0 && updates_per_second > 0.0
        ),
        "the first update after a full second brings the stats of the raw updates, got {stats:?}"
    );
}
