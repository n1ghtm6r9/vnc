use super::table_fields;
use crate::{MEDIA_SOFTWARE, ProtoField};

pub(crate) fn audio_blob_fields(ssrc: u32, ntp_time: u64) -> Vec<ProtoField> {
    let mut fields = vec![
        ProtoField::varint(1, 1),
        ProtoField::varint(2, 1),
        ProtoField::message(
            3,
            &[
                ProtoField::varint(1, ssrc.into()),
                ProtoField::varint(2, 0),
                ProtoField::varint(3, 0),
                ProtoField::varint(4, 7807),
                ProtoField::varint(5, 0),
                ProtoField::varint(6, 0),
            ],
        ),
        ProtoField::text(6, MEDIA_SOFTWARE),
        ProtoField::varint(8, 0),
    ];
    fields.extend(table_fields());
    fields.extend([
        ProtoField::varint(13, ntp_time),
        ProtoField::varint(14, 2),
        ProtoField::varint(16, 0),
        ProtoField::varint(18, 1),
    ]);
    fields
}
