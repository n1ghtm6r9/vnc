use crate::{picture_ack_packet, test_support::hex_bytes};

#[test]
fn test_a_picture_ack_names_its_decoding_order_number() {
    assert_eq!(
        picture_ack_packet(0x73fd_5806, 0x3196).to_vec(),
        hex_bytes("80cc0003 73fd5806 00000005 00003196"),
        "the acknowledgement is an APP packet carrying the number"
    );
    assert_eq!(
        picture_ack_packet(0x8000_0001, 0xffff).to_vec(),
        hex_bytes("80cc0003 80000001 00000005 0000ffff"),
        "a source with the top bit set and the highest number"
    );
}
