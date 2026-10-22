pub fn compact_length(length: usize) -> Vec<u8> {
    let mut bytes = vec![u8::try_from(length & 0x7F).expect("seven bits fit a byte")];
    if length > 0x7F {
        bytes[0] |= 0x80;
        bytes.push(u8::try_from((length >> 7) & 0x7F).expect("seven bits fit a byte"));
    }
    if length > 0x3FFF {
        bytes[1] |= 0x80;
        bytes.push(u8::try_from(length >> 14).expect("a compact length stays below 22 bits"));
    }
    bytes
}
