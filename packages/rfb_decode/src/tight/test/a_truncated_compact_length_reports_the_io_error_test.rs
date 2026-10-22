use std::io::{Cursor, ErrorKind};

use crate::{DecodeError, read_compact_length};

#[test]
fn test_a_truncated_compact_length_reports_the_io_error() {
    for bytes in [&[][..], &[0x80][..], &[0xFF, 0xFF][..]] {
        let mut reader = Cursor::new(bytes);
        let result = read_compact_length(&mut reader);
        assert!(
            matches!(&result, Err(DecodeError::Io(error)) if error.kind() == ErrorKind::UnexpectedEof),
            "a length cut after {} bytes is an unexpected end of data, got {result:?}",
            bytes.len()
        );
    }
}
