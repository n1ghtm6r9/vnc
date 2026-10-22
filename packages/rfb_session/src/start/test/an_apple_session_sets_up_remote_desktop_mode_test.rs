use rfb::{APPLE_MODE_CONTROL, APPLE_PSEUDO_ENCODINGS, ClientMessage, PixelFormat, Rect};

use crate::test_support::{
    FakeServer, SERVER_NAME, apple_test_options, next_event, serve_apple_until_connected,
    start_with_events,
};
use crate::{Quality, SessionEvent, encodings_for_quality};

#[test]
fn test_an_apple_session_sets_up_remote_desktop_mode() {
    let server = FakeServer::start(|stream| serve_apple_until_connected(stream, 3024, 1964).1);
    let (_session, events) = start_with_events(apple_test_options(server.port()));

    let connected = next_event(&events);
    let setup = server.outcome();

    assert!(
        matches!(
            &connected,
            SessionEvent::Connected { width: 3024, height: 1964, name, apple_remote_desktop: true } if name == SERVER_NAME
        ),
        "the session connects in Apple Remote Desktop mode with the size and name of the Mac, got {connected:?}"
    );
    let screen = Rect::new(0, 0, 3024, 1964);
    let mut encodings = encodings_for_quality(Quality::Balanced);
    encodings.extend(APPLE_PSEUDO_ENCODINGS);
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
        "the client takes control, shows every display, shares the clipboard, asks for its encodings with the Apple ones, then for one full frame and for pushed updates"
    );
}
