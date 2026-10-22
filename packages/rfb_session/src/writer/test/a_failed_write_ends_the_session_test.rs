use std::{
    io::{ErrorKind, Read},
    net::{Shutdown, TcpListener, TcpStream},
    sync::mpsc,
    time::Duration,
};

use rfb::Rect;

use crate::{ClientCommand, Outgoing, SessionWriter};

#[test]
fn test_a_failed_write_ends_the_session() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a local port is free");
    let address = listener.local_addr().expect("the listener has an address");
    let socket = TcpStream::connect(address).expect("the client reaches the local listener");
    let (_server, _) = listener.accept().expect("the listener takes the client");
    let mut reader_side = socket
        .try_clone()
        .expect("the socket can be shared with the reader");
    socket
        .shutdown(Shutdown::Write)
        .expect("the write half can be closed");
    let (outgoing, queue) = mpsc::channel();
    outgoing
        .send(Outgoing::Command(ClientCommand::RefreshScreen))
        .expect("the writer queue takes a message");
    drop(outgoing);

    SessionWriter::new(socket, queue, Rect::new(0, 0, 8, 8), None).run();

    reader_side
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("the reader side takes a timeout");
    let read = reader_side.read(&mut [0u8; 1]);
    let still_waiting = matches!(
        &read,
        Err(error) if matches!(error.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock)
    );
    assert!(
        !still_waiting,
        "after a failed write the reader stops waiting for the server, got {read:?}"
    );
}
