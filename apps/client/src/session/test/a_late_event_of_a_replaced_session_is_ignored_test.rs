use std::{net::TcpListener, sync::mpsc, thread, time::Duration};

use rfb_session::SessionEvent;
use winit::dpi::PhysicalSize;

use crate::{AppEvent, ConnectRequest, ImageQuality, SessionSlot, SoundPlace, connect_options};

#[test]
fn test_a_late_event_of_a_replaced_session_is_ignored() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a local port is free");
    let port = listener
        .local_addr()
        .expect("the listener has an address")
        .port();
    thread::spawn(move || {
        for stream in listener.incoming().take(2) {
            drop(stream);
        }
    });
    let request = ConnectRequest {
        host: "127.0.0.1".to_owned(),
        port,
        username: "bogdan".to_owned(),
        password: "secret".to_owned(),
        remember_password: false,
        apple_remote_desktop: false,
        high_performance: false,
        retina: true,
    };
    let (sender, events) = mpsc::channel();
    let first_sender = sender.clone();
    let mut slot = SessionSlot::default();

    slot.start(connect_options(&request, ImageQuality::Balanced, SoundPlace::Here, PhysicalSize::new(1280, 720)), move |event| {
        let _ = first_sender.send(event);
    });
    let AppEvent::Session {
        generation: first,
        event: first_event,
    } = events
        .recv_timeout(Duration::from_secs(5))
        .expect("the first session ends")
    else {
        panic!("a session only sends session events");
    };
    slot.start(connect_options(&request, ImageQuality::Balanced, SoundPlace::Here, PhysicalSize::new(1280, 720)), move |event| {
        let _ = sender.send(event);
    });
    let first_is_current = slot.is_current(first);
    let AppEvent::Session {
        generation: second, ..
    } = events
        .recv_timeout(Duration::from_secs(5))
        .expect("the second session ends")
    else {
        panic!("a session only sends session events");
    };
    let second_is_current = slot.is_current(second);
    slot.stop();

    assert!(
        matches!(first_event, SessionEvent::Disconnected { error: Some(_) }),
        "a server that hangs up at once ends the first session, got {first_event:?}"
    );
    assert!(
        !first_is_current,
        "once a second session starts, events of the first one are stale"
    );
    assert!(
        second_is_current,
        "events of the running session are current"
    );
    assert!(
        !slot.is_current(second),
        "after a stop no event is current"
    );
}
