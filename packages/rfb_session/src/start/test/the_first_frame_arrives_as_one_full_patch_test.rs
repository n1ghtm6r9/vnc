use rfb::{Rect, write_framebuffer_update_header};

use crate::test_support::{
    FakeServer, next_event, serve_until_connected, start_with_events, test_options, write_raw_rect,
};
use crate::{Patch, SessionEvent};

#[test]
fn test_the_first_frame_arrives_as_one_full_patch() {
    let background = 0xFF20_4060;
    let window = 0xFFE0_C0A0;
    let server = FakeServer::start(move |stream| {
        serve_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 2)
            .expect("the fake server starts the first update");
        write_raw_rect(stream, Rect::new(0, 0, 64, 48), background);
        write_raw_rect(stream, Rect::new(8, 8, 16, 16), window);
    });
    let (_session, events) = start_with_events(test_options(server.port()));

    let connected = next_event(&events);
    let frame = next_event(&events);
    server.outcome();

    let expected_pixels: Vec<u32> = (0..48u16)
        .flat_map(|y| {
            (0..64u16).map(move |x| {
                if (8..24).contains(&x) && (8..24).contains(&y) {
                    window
                } else {
                    background
                }
            })
        })
        .collect();
    assert!(
        matches!(connected, SessionEvent::Connected { .. }),
        "the session connects first, got {connected:?}"
    );
    let SessionEvent::FrameUpdated { patches } = frame else {
        panic!("the first update arrives as FrameUpdated, got {frame:?}");
    };
    assert_eq!(
        patches,
        vec![Patch {
            rect: Rect::new(0, 0, 64, 48),
            pixels: expected_pixels,
        }],
        "two overlapping rects of the first update arrive as one full-screen patch with both drawn"
    );
}
