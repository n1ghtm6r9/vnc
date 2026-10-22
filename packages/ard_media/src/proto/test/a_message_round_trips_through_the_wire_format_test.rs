use crate::{ProtoField, ProtoValue, decode_proto, encode_proto, proto_bytes, proto_varint};

#[test]
fn test_a_message_round_trips_through_the_wire_format() {
    let inner = [
        ProtoField::varint(1, 0xf1e2_d3c4),
        ProtoField::varint(4, 3840),
    ];
    let fields = vec![
        ProtoField::varint(1, 1),
        ProtoField::message(5, &inner),
        ProtoField::text(6, "Viceroy 1.7.0"),
        ProtoField::varint(13, 0xee78_011e_53bb_b000),
        ProtoField {
            number: 7,
            value: ProtoValue::Fixed32([1, 2, 3, 4]),
        },
        ProtoField {
            number: 8,
            value: ProtoValue::Fixed64([9; 8]),
        },
    ];

    let decoded = decode_proto(&encode_proto(&fields)).expect("an encoded message decodes");
    let nested = decode_proto(proto_bytes(&decoded, 5).expect("the nested message is there"))
        .expect("the nested message decodes");

    assert_eq!(decoded, fields, "every field comes back in wire order");
    assert_eq!(
        proto_varint(&nested, 4),
        Some(3840),
        "a nested varint is readable"
    );
    assert_eq!(
        proto_varint(&decoded, 13),
        Some(0xee78_011e_53bb_b000),
        "a 64-bit time survives"
    );
    assert_eq!(
        proto_varint(&decoded, 6),
        None,
        "text is not read as a number"
    );
}
