pub fn mvs_partial_update(red_limit: u8, blue_limit: u8, body: &[u8]) -> Vec<u8> {
    let mut update = vec![0x01, red_limit, blue_limit];
    update.extend_from_slice(body);
    update
}
