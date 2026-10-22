use std::io::{self, Cursor};

use crate::{DecodeError, read_scratch};

#[test]
fn test_reading_past_the_end_of_the_stream_is_an_io_error() {
    let mut reader = Cursor::new(vec![1u8, 2, 3]);
    let mut buffer = Vec::new();
    match read_scratch(&mut reader, &mut buffer, 8) {
        Err(DecodeError::Io(error)) => assert_eq!(
            error.kind(),
            io::ErrorKind::UnexpectedEof,
            "a short stream reports an unexpected end"
        ),
        other => panic!("a short stream must report an io error, got {other:?}"),
    }
}
