use crate::{ProtoField, encode_proto};

#[test]
fn test_varints_use_seven_bits_a_byte() {
    assert_eq!(
        encode_proto(&[ProtoField::varint(1, 1)]),
        [0x08, 0x01],
        "a one-byte varint"
    );
    assert_eq!(
        encode_proto(&[ProtoField::varint(2, 300)]),
        [0x10, 0xac, 0x02],
        "300 takes two bytes, the low seven bits first"
    );
    assert_eq!(
        encode_proto(&[ProtoField::text(3, "Mac")]),
        [0x1a, 0x03, b'M', b'a', b'c'],
        "text is length-delimited"
    );
    assert_eq!(
        encode_proto(&[ProtoField::varint(1, u64::MAX)]).len(),
        11,
        "the largest varint takes ten bytes after its tag"
    );
}
