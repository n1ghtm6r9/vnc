use std::io::Cursor;

use plist::Value;

use super::fixtures::fixed_secrets;
use crate::{build_offer, decode_proto, proto_bytes, proto_varint, zlib_inflate};

fn stream(bytes: &[u8]) -> plist::Dictionary {
    Value::from_reader(Cursor::new(bytes))
        .expect("each stream offer is a property list")
        .into_dictionary()
        .expect("each stream offer is a dictionary")
}

#[test]
fn test_the_offer_carries_header_keys_and_both_streams() {
    let secrets = fixed_secrets();
    let ntp = 0xe5d4_c3b2_a190_8070;

    let offer = build_offer(&secrets, ntp).expect("the offer fits in one message");
    let audio_length = usize::from(u16::from_be_bytes([offer[10], offer[11]]));
    let video_length = usize::from(u16::from_be_bytes([offer[12], offer[13]]));
    let audio_start = 36 + 92;
    let video_keys = audio_start + audio_length;
    let video_start = video_keys + 92;
    let audio = stream(&offer[audio_start..video_keys]);
    let video = stream(&offer[video_start..video_start + video_length]);
    let audio_blob = decode_proto(
        &zlib_inflate(
            audio["avcMediaStreamNegotiatorMediaBlob"]
                .as_data()
                .expect("a blob"),
        )
        .expect("the blob unpacks"),
    )
    .expect("the blob decodes");
    let audio_stream =
        decode_proto(proto_bytes(&audio_blob, 3).expect("audio parameters")).expect("they decode");

    assert_eq!(&offer[..2], [0x1c, 0], "the message type is 0x1c");
    assert_eq!(
        usize::from(u16::from_be_bytes([offer[2], offer[3]])),
        offer.len() - 4,
        "the length counts what follows it"
    );
    assert_eq!(
        &offer[4..10],
        [0, 3, 0, 0, 0, 0x0d],
        "version 3 and flags 0xd"
    );
    assert_eq!(
        &offer[14..20],
        [0; 6],
        "the three unused stream sizes are zero"
    );
    assert_eq!(
        offer[20..36],
        secrets.session_id,
        "the session id follows the header"
    );
    assert_eq!(
        offer[36..audio_start],
        secrets.audio_keys,
        "the audio keys come before the audio offer"
    );
    assert_eq!(
        offer[video_keys..video_start],
        secrets.video_keys,
        "the video keys come before the video offer"
    );
    assert_eq!(
        video_start + video_length,
        offer.len(),
        "the video offer ends the message"
    );
    assert_eq!(
        audio["avcMediaStreamNegotiatorMode"].as_unsigned_integer(),
        Some(8),
        "the audio stream has mode 8"
    );
    assert_eq!(
        video["avcMediaStreamNegotiatorMode"].as_unsigned_integer(),
        Some(7),
        "the video stream has mode 7"
    );
    assert_eq!(
        audio["avcMediaStreamOptionCallID"].as_string(),
        Some(secrets.call_ids[0].as_str()),
        "the audio offer has the first call id"
    );
    assert_eq!(
        video["avcMediaStreamOptionCallID"].as_string(),
        Some(secrets.call_ids[1].as_str()),
        "the video offer has the second call id"
    );
    assert_eq!(
        proto_varint(&audio_stream, 1),
        Some(0x1122_3344),
        "the audio offer names our audio source"
    );
    assert_eq!(
        proto_varint(&audio_blob, 13),
        Some(ntp),
        "the offer carries the time"
    );
    assert_eq!(
        decode_proto(
            audio["avcMediaStreamOptionRemoteEndpointInfo"]
                .as_data()
                .expect("endpoint")
        )
        .expect("it decodes")
        .len(),
        5,
        "the viewer describes itself in five fields"
    );
}
