use std::{net::TcpListener, time::Duration};

use socket2::SockRef;

use crate::connect_tcp;

#[test]
fn test_the_session_socket_probes_a_silent_link() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a local port is free");
    let port = listener
        .local_addr()
        .expect("the listener has an address")
        .port();

    let socket = connect_tcp("127.0.0.1", port, Duration::from_secs(5))
        .expect("the client reaches the local listener");

    assert!(
        SockRef::from(&socket)
            .keepalive()
            .expect("the keepalive option can be read"),
        "the session socket sends keepalive probes, so a Mac that fell asleep ends the session"
    );
}
