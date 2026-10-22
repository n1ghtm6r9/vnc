use std::io::Cursor;

use crate::read_scratch;

#[test]
fn test_reading_into_the_scratch_buffer_reuses_it_for_a_shorter_read() {
    let mut reader = Cursor::new((1..=8u8).collect::<Vec<u8>>());
    let mut buffer = Vec::new();
    let first = read_scratch(&mut reader, &mut buffer, 4)
        .expect("four bytes are available")
        .to_vec();
    assert_eq!(
        first,
        vec![1, 2, 3, 4],
        "the first read returns the first four bytes"
    );
    assert_eq!(buffer.len(), 4, "the buffer grew to the read size");
    let second = read_scratch(&mut reader, &mut buffer, 2)
        .expect("two more bytes are available")
        .to_vec();
    assert_eq!(
        second,
        vec![5, 6],
        "the second read returns only the requested bytes"
    );
    assert_eq!(buffer.len(), 4, "a shorter read does not shrink the buffer");
}
