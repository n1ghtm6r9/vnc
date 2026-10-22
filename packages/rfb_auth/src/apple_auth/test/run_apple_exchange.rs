use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{
    AppleLogin, AppleServerOptions, AppleSessionKey, AuthError, Credentials, authenticate_apple,
    serve_apple_auth,
};

pub(super) fn run_apple_exchange(
    options: AppleServerOptions,
    credentials: &Credentials,
) -> (
    Result<AppleSessionKey, AuthError>,
    Result<AppleLogin, AuthError>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        serve_apple_auth(&mut stream, options)
    });
    let mut stream = TcpStream::connect(address).unwrap();
    let client_result = authenticate_apple(&mut stream, credentials);
    drop(stream);
    let server_result = server.join().unwrap();
    (client_result, server_result)
}
