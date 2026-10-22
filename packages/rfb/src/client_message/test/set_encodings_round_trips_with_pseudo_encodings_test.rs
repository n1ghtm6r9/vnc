use crate::{
    ClientMessage, ENCODING_CURSOR, ENCODING_JPEG_QUALITY_LEVEL_0, ENCODING_ZRLE, write_set_encodings,
};

#[test]
fn test_set_encodings_round_trips_with_pseudo_encodings() {
    let encodings = vec![ENCODING_ZRLE, ENCODING_CURSOR, ENCODING_JPEG_QUALITY_LEVEL_0 + 8];
    let mut bytes = Vec::new();
    write_set_encodings(&mut bytes, &encodings).unwrap();
    assert_eq!(&bytes[..4], &[2, 0, 0, 3], "type, padding, then the count");
    let message = ClientMessage::read(&mut &bytes[..]).unwrap();
    assert_eq!(message, ClientMessage::SetEncodings(encodings), "negative pseudo-encodings keep their sign");
}
