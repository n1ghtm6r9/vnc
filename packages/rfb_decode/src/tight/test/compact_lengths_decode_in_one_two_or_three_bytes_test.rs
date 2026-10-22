use std::io::Cursor;

use crate::read_compact_length;

#[test]
fn test_compact_lengths_decode_in_one_two_or_three_bytes() {
    let cases: [(&[u8], usize); 8] = [
        (&[0x00], 0),
        (&[0x7F], 127),
        (&[0x80, 0x01], 128),
        (&[0x90, 0x4E], 10000),
        (&[0xFF, 0x7F], 16383),
        (&[0x80, 0x80, 0x01], 16384),
        (&[0xFF, 0xFF, 0xFF], 4_194_303),
        (&[0x05, 0xAA, 0xBB], 5),
    ];
    for (bytes, expected) in cases {
        let mut reader = Cursor::new(bytes);
        let length = read_compact_length(&mut reader).expect("a compact length decodes");
        assert_eq!(length, expected, "the bytes {bytes:02X?} encode {expected}");
    }
}
