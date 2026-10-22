use ard_media::{virtual_display_for, virtual_display_message};

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, MediaCall, fake_media_starter, high_performance_test_options, next_event,
    next_media_call, next_media_start, read_client_messages, serve_high_performance_until_connected,
    start_with_media,
};

#[test]
fn test_a_window_resize_to_the_same_display_asks_for_nothing() {
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
        width: 1280,
        height: 720,
    });
    session.send(ClientCommand::ResizeDisplay {
        width: 1600,
        height: 900,
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
        MediaCall::Resize(virtual_display_message(&[virtual_display_for(1600, 900, 2)])),
        "a window size that gives the display the Mac already has asks for nothing"
    );
}
