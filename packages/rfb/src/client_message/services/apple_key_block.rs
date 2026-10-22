pub fn apple_key_block(keysym: u32, down: bool, timestamp: u32) -> [u8; 16] {
    let mut block = [0u8; 16];
    block[0] = 0xff;
    block[1] = u8::from(down);
    block[2..6].copy_from_slice(&keysym.to_be_bytes());
    block[6..10].copy_from_slice(&timestamp.to_be_bytes());
    block
}
