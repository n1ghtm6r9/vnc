use hp_media::{MediaEvent, MediaState};
use rfb::{ClientMessage, Rect};

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event, next_media_start,
    read_client_messages, serve_high_performance_until_connected, start_with_media,
};

#[test]
fn test_no_update_requests_go_out_while_the_media_streams() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
        read_client_messages(stream, 2)
    });
    let (start_media, starts) = fake_media_starter();
    let (session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let mut media = next_media_start(&starts);
    (media.emit)(MediaEvent::State(MediaState::Streaming {
        width: 64,
        height: 48,
        stripes: 1,
    }));
    next_event(&events);

    session.send(ClientCommand::RefreshScreen);
    session.send(ClientCommand::Pointer {
        x: 1,
        y: 2,
        buttons: 0,
    });
    let received = server.outcome();

    assert_eq!(
        received,
        vec![
            ClientMessage::AppleAutoUpdate {
                enabled: true,
                rect: Rect::new(0, 0, 64, 48)
            },
            ClientMessage::PointerEvent {
                buttons: 0,
                x: 1,
                y: 2
            },
        ],
        "while the media streams, a refresh only renews the pushed updates and asks for no pixels"
    );
}
