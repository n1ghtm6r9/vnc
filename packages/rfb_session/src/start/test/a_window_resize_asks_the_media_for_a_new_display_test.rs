use ard_media::{virtual_display_for, virtual_display_message};

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, MediaCall, fake_media_starter, high_performance_test_options, next_event,
    next_media_call, next_media_start, read_client_messages, serve_high_performance_until_connected,
    start_with_media,
};

#[test]
fn test_a_window_resize_asks_the_media_for_a_new_display() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (start_media, starts) = fake_media_starter();
    let (session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let media = next_media_start(&starts);

    session.send(ClientCommand::ResizeDisplay {
        width: 1920,
        height: 1200,
    });
    session.send(ClientCommand::Pointer {
        x: 1,
        y: 2,
        buttons: 0,
    });
    let call = next_media_call(&media);
    server.outcome();

    assert_eq!(
        call,
        MediaCall::Resize(virtual_display_message(&[virtual_display_for(1920, 1200, 2)])),
        "a new window size becomes a virtual display of that size at the session's scale"
    );
}
