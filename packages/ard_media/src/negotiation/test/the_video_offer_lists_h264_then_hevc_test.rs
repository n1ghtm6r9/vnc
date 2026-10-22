use std::io::Cursor;

use plist::Value;

use super::fixtures::fixed_secrets;
use crate::{ProtoValue, build_offer, decode_proto, proto_bytes, proto_varint, zlib_inflate};

#[test]
fn test_the_video_offer_lists_h264_then_hevc() {
    let offer = build_offer(&fixed_secrets(), 1).expect("the offer fits in one message");
    let audio_length = usize::from(u16::from_be_bytes([offer[10], offer[11]]));
    let video_start = 36 + 92 + audio_length + 92;
    let video = Value::from_reader(Cursor::new(&offer[video_start..]))
        .expect("the video offer is a property list")
        .into_dictionary()
        .expect("it is a dictionary");
    let blob = decode_proto(
        &zlib_inflate(
            video["avcMediaStreamNegotiatorMediaBlob"]
                .as_data()
                .expect("a blob"),
        )
        .expect("it unpacks"),
    )
    .expect("it decodes");
    let stream =
        decode_proto(proto_bytes(&blob, 5).expect("video parameters")).expect("they decode");
    let formats: Vec<Vec<crate::ProtoField>> = stream
        .iter()
        .filter(|field| field.number == 3)
        .map(|field| match &field.value {
            ProtoValue::Bytes(bytes) => decode_proto(bytes).expect("a format decodes"),
            other => panic!("a format is a message, not {other:?}"),
        })
        .collect();

    assert_eq!(
        proto_varint(&stream, 1),
        Some(0xf1e2_d3c4),
        "the video offer names our video source"
    );
    assert_eq!(formats.len(), 2, "two formats are offered");
    assert_eq!(
        proto_varint(&formats[0], 1),
        Some(123),
        "H.264 comes first, as in Apple's offer"
    );
    assert_eq!(proto_varint(&formats[1], 1), Some(100), "HEVC comes second");
    assert_eq!(
        proto_bytes(&formats[1], 3),
        Some(crate::FEATURES_HEVC.as_bytes()),
        "the HEVC format carries Apple's feature string"
    );
    assert_eq!(
        blob.iter().filter(|field| field.number == 9).count(),
        10,
        "the ten-row table follows the stream"
    );
}
