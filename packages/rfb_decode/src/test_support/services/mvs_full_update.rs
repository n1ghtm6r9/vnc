pub fn mvs_full_update(threshold1: u8, threshold2: u8, header: &[u8], body: &[u8]) -> Vec<u8> {
    let body_offset = u32::try_from(6 + header.len())
        .expect("a test header is short")
        .to_be_bytes();
    let mut update = vec![
        0x00,
        threshold1,
        threshold2,
        body_offset[1],
        body_offset[2],
        body_offset[3],
    ];
    update.extend_from_slice(header);
    update.extend_from_slice(body);
    update
}
