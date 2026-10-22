pub fn annex_b_unit(kind: u8, body: &[u8]) -> Vec<u8> {
    let mut data = vec![0, 0, 0, 1, kind << 1, 1];
    data.extend_from_slice(body);
    data
}
