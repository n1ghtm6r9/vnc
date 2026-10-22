pub fn picture_ack_packet(ssrc: u32, don: u16) -> [u8; 16] {
    let mut packet = [0u8; 16];
    let words = [0x80cc_0003, ssrc, 0x0000_0005, u32::from(don)];
    let (slots, _) = packet.as_chunks_mut::<4>();
    for (slot, word) in slots.iter_mut().zip(words) {
        *slot = word.to_be_bytes();
    }
    packet
}
