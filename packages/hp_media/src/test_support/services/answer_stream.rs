use ard_media::{KEY_MEDIA_BLOB, KEY_REMOTE_ENDPOINT, ProtoField, encode_proto};
use plist::{Dictionary, Value};

use super::zlib;

pub fn answer_stream(blob: &[ProtoField]) -> Vec<u8> {
    let mut stream = Dictionary::new();
    stream.insert(
        KEY_MEDIA_BLOB.into(),
        Value::Data(zlib(&encode_proto(blob))),
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
