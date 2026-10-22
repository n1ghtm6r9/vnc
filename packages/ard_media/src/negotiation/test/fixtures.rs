use plist::{Dictionary, Value};

use crate::{
    KEY_MEDIA_BLOB, KEY_REMOTE_ENDPOINT, MediaSecrets, ProtoField, encode_proto, zlib_deflate,
};

pub(super) fn message1(strings: &[&str]) -> Vec<u8> {
    let mut body = vec![0, 2, 0, 0, 0, 1, 0, 0, 0, 0];
    for text in strings {
        body.extend_from_slice(&(text.len() as u16 + 1).to_be_bytes());
    }
    for text in strings {
        body.extend_from_slice(text.as_bytes());
        body.push(0);
    }
    body.extend_from_slice(&[0; 4]);
    with_length(body)
}

pub(super) fn with_length(body: Vec<u8>) -> Vec<u8> {
    let mut payload = (body.len() as u16).to_be_bytes().to_vec();
    payload.extend(body);
    payload
}

pub(super) fn answer_stream(blob: &[ProtoField]) -> Vec<u8> {
    let mut stream = Dictionary::new();
    stream.insert(
        KEY_MEDIA_BLOB.into(),
        Value::Data(zlib_deflate(&encode_proto(blob))),
    );
    stream.insert(
        KEY_REMOTE_ENDPOINT.into(),
        Value::Data(encode_proto(&[
            ProtoField::varint(1, 0),
            ProtoField::text(3, "Test1,1"),
        ])),
    );
    let mut bytes = Vec::new();
    Value::Dictionary(stream)
        .to_writer_binary(&mut bytes)
        .expect("a property list is written into memory");
    bytes
}

pub(super) fn answer_payload(audio: &[u8], video: &[u8]) -> Vec<u8> {
    let mut body = vec![0, 2, 0, 2, 0, 0, 0, 1];
    body.extend_from_slice(&(audio.len() as u16).to_be_bytes());
    body.extend_from_slice(&(video.len() as u16).to_be_bytes());
    body.extend_from_slice(&[0; 6]);
    body.extend_from_slice(audio);
    body.extend_from_slice(video);
    with_length(body)
}

pub(super) fn audio_blob(ssrc: u32) -> Vec<ProtoField> {
    vec![
        ProtoField::varint(1, 1),
        ProtoField::message(
            3,
            &[ProtoField::varint(1, ssrc.into()), ProtoField::varint(6, 1)],
        ),
        ProtoField::text(6, "Viceroy 1.7.0"),
    ]
}

pub(super) fn video_blob(fields: &[ProtoField]) -> Vec<ProtoField> {
    vec![ProtoField::varint(1, 1), ProtoField::message(5, fields)]
}

pub(super) fn video_fields(ssrc: u32) -> Vec<ProtoField> {
    vec![
        ProtoField::varint(1, ssrc.into()),
        ProtoField::varint(4, 3840),
        ProtoField::varint(5, 2160),
        ProtoField::varint(6, 4),
    ]
}

pub(super) fn fixed_secrets() -> MediaSecrets {
    MediaSecrets {
        session_id: std::array::from_fn(|index| 0xa0 + index as u8),
        audio_keys: std::array::from_fn(|index| index as u8),
        video_keys: std::array::from_fn(|index| 0xff - index as u8),
        audio_ssrc: 0x1122_3344,
        video_ssrc: 0xf1e2_d3c4,
        call_ids: [
            "11111111-2222-4333-8444-555555555555".into(),
            "AAAAAAAA-BBBB-4CCC-8DDD-EEEEEEEEEEEE".into(),
        ],
    }
}
