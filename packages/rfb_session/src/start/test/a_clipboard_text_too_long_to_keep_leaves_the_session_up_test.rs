use rfb::{MAX_CUT_TEXT_LENGTH, write_server_cut_text};

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, next_event, serve_until_connected, start_with_events, test_options,
};

#[test]
fn test_a_clipboard_text_too_long_to_keep_leaves_the_session_up() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        let huge = vec![b'a'; MAX_CUT_TEXT_LENGTH as usize + 1];
        write_server_cut_text(stream, &huge).expect("the fake server sends a huge clipboard");
        write_server_cut_text(stream, b"short").expect("the fake server sends a short clipboard");
    });
    let (_session, events) = start_with_events(test_options(server.port()));

    let connected = next_event(&events);
    let clipboard = next_event(&events);
    server.outcome();

    assert!(
        matches!(connected, SessionEvent::Connected { .. }),
        "the session connects first, got {connected:?}"
    );
    assert!(
        matches!(&clipboard, SessionEvent::ClipboardText(text) if text == "short"),
        "a Mac clipboard over 16 MiB is passed over and the session goes on, got {clipboard:?}"
    );
}
