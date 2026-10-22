use crate::{ENDPOINT_BUILD, ENDPOINT_MODEL, ENDPOINT_VERSION, ProtoField};

pub(crate) fn endpoint_fields() -> Vec<ProtoField> {
    vec![
        ProtoField::varint(1, 0),
        ProtoField::varint(2, 1),
        ProtoField::text(3, ENDPOINT_MODEL),
        ProtoField::text(4, ENDPOINT_VERSION),
        ProtoField::text(5, ENDPOINT_BUILD),
    ]
}
