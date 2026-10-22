use std::net::{Ipv4Addr, UdpSocket};

use crate::{
    MediaFactories, MediaStartError, launch_media,
    test_support::{TestMac, event_channel, fixed_secrets, new_fake_decoder, sound_channel},
};

#[test]
fn test_a_media_port_in_use_is_reported_at_start() {
    let taken = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("a free loopback port");
    let mac = TestMac::new();
    let mut options = mac.options();
    options.ports.local[1] = taken
        .local_addr()
        .expect("a bound socket has an address")
        .port();
    let (on_event, _events) = event_channel();

    let outcome = launch_media(
        options,
        on_event,
        MediaFactories {
            new_decoder: new_fake_decoder,
            new_secrets: fixed_secrets,
            new_sound: sound_channel().0,
        },
    );

    assert!(
        matches!(outcome, Err(MediaStartError::Bind(_))),
        "a video port someone else holds is reported, got {outcome:?}"
    );
}
