use rfb::ClientMessage;

use crate::test_support::{
    FakeServer, next_event, read_client_messages, serve_until_connected, start_with_events,
    test_options,
};
use crate::{ClientCommand, SessionEvent};

#[test]
fn test_clipboard_text_goes_out_as_latin1() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (session, events) = start_with_events(test_options(server.port()));
    let connected = next_event(&events);

    session.send(ClientCommand::ClipboardText(
        "caf\u{e9} \u{bd} \u{436}".to_owned(),
    ));
    let received = server.outcome();

    assert!(
        matches!(connected, SessionEvent::Connected { .. }),
        "the session connects first, got {connected:?}"
    );
    assert_eq!(
        received,
        vec![ClientMessage::ClientCutText {
            text: b"caf\xe9 \xbd ?".to_vec()
        }],
        "clipboard text goes to the Mac as Latin-1, characters outside it become question marks"
    );
}
