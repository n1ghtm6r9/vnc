use rfb::ClientMessage;

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, next_event, read_client_messages, serve_until_connected, start_with_events,
    test_options,
};

#[test]
fn test_an_input_source_choice_skips_a_plain_vnc_server() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (session, events) = start_with_events(test_options(server.port()));
    next_event(&events);

    session.send(ClientCommand::SetInputSource("com.apple.keylayout.ABC".to_owned()));
    session.send(ClientCommand::RefreshScreen);
    let received = server.outcome();

    assert!(
        matches!(
            received.as_slice(),
            [ClientMessage::FramebufferUpdateRequest { incremental: false, .. }]
        ),
        "a plain VNC server gets the screen request and no Apple input source message, got {received:?}"
    );
}
