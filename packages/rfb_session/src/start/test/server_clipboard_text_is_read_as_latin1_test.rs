use rfb::write_server_cut_text;

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, next_event, serve_until_connected, start_with_events, test_options,
};

#[test]
fn test_server_clipboard_text_is_read_as_latin1() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        write_server_cut_text(stream, b"caf\xe9 \xbd")
            .expect("the fake server sends its clipboard");
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
        matches!(&clipboard, SessionEvent::ClipboardText(text) if text == "caf\u{e9} \u{bd}"),
        "clipboard bytes of the Mac are read as Latin-1, got {clipboard:?}"
    );
}
