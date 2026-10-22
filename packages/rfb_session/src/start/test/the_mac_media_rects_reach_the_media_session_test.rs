use rfb::{
    ENCODING_APPLE_HIGH_PERFORMANCE, ENCODING_APPLE_MEDIA_STREAM, write_framebuffer_update_header,
};

use crate::test_support::{
    FakeServer, MediaCall, fake_media_starter, high_performance_test_options, next_event,
    next_media_call, next_media_start, serve_high_performance_until_connected, start_with_media,
    write_media_rect,
};

#[test]
fn test_the_mac_media_rects_reach_the_media_session() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 2).expect("the fake server starts an update");
        write_media_rect(stream, ENCODING_APPLE_MEDIA_STREAM, b"offer");
        write_media_rect(stream, ENCODING_APPLE_HIGH_PERFORMANCE, b"answer");
    });
    let (start_media, starts) = fake_media_starter();
    let (_session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let media = next_media_start(&starts);

    let calls = [next_media_call(&media), next_media_call(&media)];

    assert_eq!(
        calls,
        [
            MediaCall::Message1(b"\0\x05offer".to_vec()),
            MediaCall::Answer(b"\0\x06answer".to_vec()),
        ],
        "rect 1110 reaches the media as Message 1 and rect 1010 as the answer, each with its length"
    );
}
