use crate::{read_i32, read_u8, read_u16, read_u32};

#[test]
fn test_numbers_are_read_big_endian() {
    let bytes = [0x07, 0x12, 0x34, 0xde, 0xad, 0xbe, 0xef, 0xff, 0xff, 0xff, 0x11];
    let mut reader = &bytes[..];
    assert_eq!(read_u8(&mut reader).unwrap(), 0x07, "a single byte is read as it is");
    assert_eq!(read_u16(&mut reader).unwrap(), 0x1234, "the first byte of a u16 is the high one");
    assert_eq!(read_u32(&mut reader).unwrap(), 0xdead_beef, "the first byte of a u32 is the high one");
    assert_eq!(read_i32(&mut reader).unwrap(), -239, "a signed i32 keeps its sign, as pseudo-encodings need");
}
