use std::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
};

use super::FakeServer;
use crate::test_support::{EVENT_TIMEOUT, drain_until_closed};

impl<T: Send + 'static> FakeServer<T> {
    pub fn start(script: impl FnOnce(&mut TcpStream) -> T + Send + 'static) -> FakeServer<T> {
        let listener =
            TcpListener::bind("127.0.0.1:0").expect("a local port is free for the fake server");
        let port = listener
            .local_addr()
            .expect("the fake server knows its port")
            .port();
        let (report, outcome) = mpsc::channel();
        thread::spawn(move || {
            let (mut stream, _) = listener
                .accept()
                .expect("the client connects to the fake server");
            stream
                .set_read_timeout(Some(EVENT_TIMEOUT))
                .expect("the fake server limits how long it waits for the client");
            let _ = report.send(script(&mut stream));
            drain_until_closed(&mut stream);
        });
        FakeServer { port, outcome }
    }
}
