use plist::{Dictionary, Value};

use super::endpoint_fields;
use crate::{
    KEY_CALL_ID, KEY_MEDIA_BLOB, KEY_NEGOTIATOR_MODE, KEY_REMOTE_ENDPOINT, ProtoField,
    encode_proto, zlib_deflate,
};

pub(crate) fn stream_offer(mode: u64, blob: &[ProtoField], call_id: &str) -> Vec<u8> {
    let mut offer = Dictionary::new();
    offer.insert(
        KEY_REMOTE_ENDPOINT.into(),
        Value::Data(encode_proto(&endpoint_fields())),
    );
    offer.insert(KEY_NEGOTIATOR_MODE.into(), Value::Integer(mode.into()));
    offer.insert(
        KEY_MEDIA_BLOB.into(),
        Value::Data(zlib_deflate(&encode_proto(blob))),
    );
    offer.insert(KEY_CALL_ID.into(), Value::String(call_id.into()));
    let mut bytes = Vec::new();
    Value::Dictionary(offer)
        .to_writer_binary(&mut bytes)
        .expect("writing a property list into memory cannot fail");
    bytes
}
