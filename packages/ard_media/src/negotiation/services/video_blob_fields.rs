use super::{payload_entry, table_fields};
use crate::{
    FEATURES_H264, FEATURES_HEVC, MEDIA_SOFTWARE, PAYLOAD_TYPE_H264, PAYLOAD_TYPE_HEVC, ProtoField,
};

pub(crate) fn video_blob_fields(ssrc: u32, ntp_time: u64) -> Vec<ProtoField> {
    let stream = [
        ProtoField::varint(1, ssrc.into()),
        ProtoField::varint(2, 0),
        ProtoField::message(
            3,
            &payload_entry(PAYLOAD_TYPE_H264, &[1, 2, 1, 2], FEATURES_H264, 1),
        ),
        ProtoField::message(
            3,
            &payload_entry(PAYLOAD_TYPE_HEVC, &[1, 2], FEATURES_HEVC, 14),
        ),
        ProtoField::varint(6, 4),
        ProtoField::varint(7, 1),
        ProtoField::varint(8, 255),
        ProtoField::varint(9, 1),
        ProtoField::varint(12, 1),
    ];
    let mut fields = vec![
        ProtoField::varint(1, 1),
        ProtoField::varint(2, 1),
        ProtoField::message(5, &stream),
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
