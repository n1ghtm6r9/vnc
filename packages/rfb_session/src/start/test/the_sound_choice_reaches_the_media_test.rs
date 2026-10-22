use crate::ClientCommand;
use crate::test_support::{
    FakeServer, MediaCall, fake_media_starter, high_performance_test_options, next_event,
    next_media_call, next_media_start, read_client_messages, serve_high_performance_until_connected,
    start_with_media,
};

#[test]
fn test_the_sound_choice_reaches_the_media() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (start_media, starts) = fake_media_starter();
    let (session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let media = next_media_start(&starts);

    session.send(ClientCommand::PlaySound(false));
    session.send(ClientCommand::Pointer {
        x: 1,
        y: 2,
        buttons: 0,
    });
    let call = next_media_call(&media);
    server.outcome();

    assert_eq!(
        call,
        MediaCall::PlaySound(false),
        "moving the sound to the Mac tells the media to stop playing it here"
    );
}
