use crate::ProtoField;

pub(crate) fn payload_entry(
    payload_type: u8,
    variants: &[u64],
    features: &str,
    last: u64,
) -> Vec<ProtoField> {
    let mut fields = vec![ProtoField::varint(1, payload_type.into())];
    for variant in variants {
        fields.push(ProtoField::message(
            2,
            &[
                ProtoField::varint(1, 1),
                ProtoField::varint(2, *variant),
                ProtoField::varint(3, 50115),
                ProtoField::varint(4, 0),
            ],
        ));
    }
    fields.push(ProtoField::text(3, features));
    fields.push(ProtoField::varint(4, last));
    fields
}
