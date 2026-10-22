use std::mem::{offset_of, size_of};

use crate::CuvidSourceDataPacket;

#[test]
fn test_the_source_data_packet_matches_the_c_layout() {
    let fields = [
        ("flags", offset_of!(CuvidSourceDataPacket, flags), 0),
        (
            "payload_size",
            offset_of!(CuvidSourceDataPacket, payload_size),
            4,
        ),
        ("payload", offset_of!(CuvidSourceDataPacket, payload), 8),
        (
            "timestamp",
            offset_of!(CuvidSourceDataPacket, timestamp),
            16,
        ),
    ];

    for (name, actual, expected) in fields {
        assert_eq!(
            actual, expected,
            "{name} sits where CUVIDSOURCEDATAPACKET keeps it"
        );
    }
    assert_eq!(
        size_of::<CuvidSourceDataPacket>(),
        24,
        "the packet is as big as CUVIDSOURCEDATAPACKET"
    );
}
