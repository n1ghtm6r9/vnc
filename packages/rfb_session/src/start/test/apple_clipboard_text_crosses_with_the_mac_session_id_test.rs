use rfb::{ClientMessage, write_apple_clipboard};

use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};
use crate::{ClientCommand, SessionEvent};

#[test]
fn test_apple_clipboard_text_crosses_with_the_mac_session_id() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_apple_clipboard(stream, 77, "с Мака").expect("the fake server sends its pasteboard");
        read_client_messages(stream, 1)
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let from_mac = next_event(&events);
    session.send(ClientCommand::ClipboardText("на Мак".to_owned()));
    let received = server.outcome();

    assert!(
        matches!(&from_mac, SessionEvent::ClipboardText(text) if text == "с Мака"),
        "the Cyrillic pasteboard text of the Mac reaches the app, got {from_mac:?}"
    );
    assert_eq!(
        received,
        vec![ClientMessage::AppleClipboard {
            session_id: 77,
            text: Some("на Мак".to_owned())
        }],
        "the text of the app goes to the Mac as a pasteboard under the session the Mac named"
    );
}
