pub(crate) fn write_varint(out: &mut Vec<u8>, mut value: u64) {
    while value > 0x7f {
        out.push((value & 0x7f) as u8 | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}
