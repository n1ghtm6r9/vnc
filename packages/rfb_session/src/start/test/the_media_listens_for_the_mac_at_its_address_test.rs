use std::net::{IpAddr, Ipv4Addr};

use hp_media::{MAC_MEDIA_PORTS, MediaOptions};

use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event, next_media_start,
    serve_high_performance_until_connected, start_with_media,
};

#[test]
fn test_the_media_listens_for_the_mac_at_its_address() {
    let server =
        FakeServer::start(|stream| serve_high_performance_until_connected(stream, 64, 48));
    let (start_media, starts) = fake_media_starter();
    let (_session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);

    next_event(&events);
    let media = next_media_start(&starts);
    server.outcome();

    assert_eq!(
        media.options,
        MediaOptions {
            server: IpAddr::V4(Ipv4Addr::LOCALHOST),
            ports: MAC_MEDIA_PORTS,
            play_sound: true,
        },
        "the media takes the picture and the sound from the address of the Mac, on the ports the Mac sends to"
    );
}
