use crate::decode_proto;

#[test]
fn test_a_truncated_or_grouped_message_is_refused() {
    let cases: [(&[u8], &str); 6] = [
        (&[0x08], "a varint field without its value"),
        (&[0x0a, 0x05, 1, 2], "a length past the end"),
        (&[0x0b], "a group start"),
        (&[0x0e, 0], "an unassigned wire type"),
        (&[0x00, 0], "field number zero"),
        (
            &[
                0x08, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02,
            ],
            "a varint over 64 bits",
        ),
    ];
    for (bytes, meaning) in cases {
        assert!(decode_proto(bytes).is_err(), "{meaning} is refused");
    }
}
