use ard_media::{virtual_display_for, virtual_display_message};
use rfb::{
    APPLE_MODE_CONTROL, APPLE_PSEUDO_ENCODINGS, ClientMessage, ENCODING_APPLE_HIGH_PERFORMANCE,
    ENCODING_APPLE_MEDIA_STREAM, PixelFormat, Rect,
};

use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event,
    serve_high_performance_until_connected, start_with_media,
};
use crate::{Quality, encodings_for_quality};

#[test]
fn test_a_high_performance_session_asks_for_a_virtual_display_and_the_media_stream() {
    let server =
        FakeServer::start(|stream| serve_high_performance_until_connected(stream, 3024, 1964));
    let (start_media, _starts) = fake_media_starter();
    let (_session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);

    next_event(&events);
    let (setup, virtual_display) = server.outcome();

    let screen = Rect::new(0, 0, 3024, 1964);
    let mut encodings = vec![ENCODING_APPLE_HIGH_PERFORMANCE];
    encodings.extend(encodings_for_quality(Quality::Balanced));
    encodings.extend(APPLE_PSEUDO_ENCODINGS);
    encodings.push(ENCODING_APPLE_MEDIA_STREAM);
    assert_eq!(
        setup,
        vec![
            ClientMessage::AppleViewerInfo,
            ClientMessage::AppleSetMode {
                mode: APPLE_MODE_CONTROL
            },
            ClientMessage::AppleSetDisplay {
                combine_all: true,
                display_id: 0
            },
            ClientMessage::AppleAutoPasteboard { enabled: true },
            ClientMessage::SetEncodings(encodings.clone()),
            ClientMessage::SetPixelFormat(PixelFormat::rgba32()),
            ClientMessage::SetEncodings(encodings),
            ClientMessage::FramebufferUpdateRequest {
                incremental: false,
                rect: screen
            },
            ClientMessage::AppleAutoUpdate {
                enabled: true,
                rect: screen
            },
        ],
        "a High Performance session asks for the media stream with 1010 first and 1110 last in its encodings"
    );
    assert_eq!(
        virtual_display,
        virtual_display_message(&[virtual_display_for(1280, 720, 2)]),
        "right after taking control the client asks the Mac for a virtual display of the wanted size and scale"
    );
}
