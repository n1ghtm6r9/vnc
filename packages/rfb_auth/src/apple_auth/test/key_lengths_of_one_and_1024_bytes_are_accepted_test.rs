use std::io::Cursor;

use super::recording_stream::RecordingStream;
use crate::{Credentials, authenticate_apple};

fn reply_length_for_key_length(key_length: u16, prime: Vec<u8>) -> usize {
    let mut server_public = vec![0; usize::from(key_length)];
    if let Some(last) = server_public.last_mut() {
        *last = 8;
    }
    let mut hello = vec![0, 2];
    hello.extend_from_slice(&key_length.to_be_bytes());
    hello.extend_from_slice(&prime);
    hello.extend_from_slice(&server_public);
    let mut stream = RecordingStream {
        input: Cursor::new(hello),
        ..Default::default()
    };
    authenticate_apple(&mut stream, &Credentials::new("bogdan", "secret")).unwrap();
    stream.writes.iter().map(Vec::len).sum()
}

#[test]
fn test_key_lengths_of_one_and_1024_bytes_are_accepted() {
    let mut largest_prime = vec![0xff; 1024];
    largest_prime[0] = 0x7f;
    assert_eq!(
        reply_length_for_key_length(1024, largest_prime),
        128 + 1024,
        "a 1024-byte key sits exactly on the cap and is accepted"
    );
    assert_eq!(
        reply_length_for_key_length(1, vec![0xfb]),
        128 + 1,
        "a 1-byte key is the smallest length and is accepted"
    );
}
