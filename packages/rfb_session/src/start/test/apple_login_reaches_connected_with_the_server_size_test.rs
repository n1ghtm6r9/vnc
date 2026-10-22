use rfb::{ClientMessage, PixelFormat, Rect};

use crate::test_support::{
    FakeServer, SERVER_NAME, next_event, serve_until_connected, start_with_events,
    test_credentials, test_options,
};
use crate::{Quality, SessionEvent, encodings_for_quality};

#[test]
fn test_apple_login_reaches_connected_with_the_server_size() {
    let server = FakeServer::start(|stream| serve_until_connected(stream, 3456, 2234));
    let (_session, events) = start_with_events(test_options(server.port()));

    let connected = next_event(&events);
    let (credentials, setup) = server.outcome();

    assert!(
        matches!(
            &connected,
            SessionEvent::Connected { width: 3456, height: 2234, name, apple_remote_desktop: false } if name == SERVER_NAME
        ),
        "the first event is Connected with the size and name from ServerInit, in plain mode, got {connected:?}"
    );
    assert_eq!(
        credentials,
        test_credentials(),
        "the Mac receives the user name and the password"
    );
    assert_eq!(
        setup,
        vec![
            ClientMessage::SetPixelFormat(PixelFormat::rgba32()),
            ClientMessage::SetEncodings(encodings_for_quality(Quality::Balanced)),
            ClientMessage::FramebufferUpdateRequest {
                incremental: false,
                rect: Rect::new(0, 0, 3456, 2234),
            },
        ],
        "after ServerInit the client asks for RGBA pixels, for its encodings and for one full frame"
    );
}
