pub fn rtp_extension(count: u16, frame: u16) -> Vec<u8> {
    let mut bytes = vec![0x93, 0x11, 0x00, 0x01];
    bytes.extend_from_slice(&count.to_be_bytes());
    bytes.extend_from_slice(&frame.to_be_bytes());
    bytes
}
