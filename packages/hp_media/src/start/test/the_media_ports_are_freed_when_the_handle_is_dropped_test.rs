use std::{
    net::{Ipv4Addr, UdpSocket},
    thread,
    time::{Duration, Instant},
};

use crate::{
    MediaFactories, launch_media,
    test_support::{
        EVENT_TIMEOUT, TestMac, event_channel, fixed_secrets, new_fake_decoder, sound_channel,
    },
};

#[test]
fn test_the_media_ports_are_freed_when_the_handle_is_dropped() {
    let mac = TestMac::new();
    let (on_event, _events) = event_channel();
    let handle = launch_media(
        mac.options(),
        on_event,
        MediaFactories {
            new_decoder: new_fake_decoder,
            new_secrets: fixed_secrets,
            new_sound: sound_channel().0,
        },
    )
    .expect("the media starts on free loopback ports");
    let ports = handle.local_ports();

    drop(handle);

    let deadline = Instant::now() + EVENT_TIMEOUT;
    for port in ports {
        while UdpSocket::bind((Ipv4Addr::LOCALHOST, port)).is_err() {
            assert!(
                Instant::now() < deadline,
                "port {port} is freed once the media threads end"
            );
            thread::sleep(Duration::from_millis(20));
        }
    }
}
