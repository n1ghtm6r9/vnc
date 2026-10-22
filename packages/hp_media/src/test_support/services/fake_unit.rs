pub fn fake_unit(kind: u8, colour: u8) -> Vec<u8> {
    vec![kind << 1, 1, colour]
}
