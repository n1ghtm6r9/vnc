use rfb::{
    APPLE_PSEUDO_ENCODINGS, ENCODING_APPLE_HIGH_PERFORMANCE, ENCODING_APPLE_MEDIA_STREAM,
    ENCODING_RAW,
};

use crate::high_performance_encodings;

#[test]
fn test_the_media_encodings_wrap_the_pixel_encodings_once() {
    let once = high_performance_encodings(&[ENCODING_RAW]);

    let twice = high_performance_encodings(&once);

    let mut expected = vec![ENCODING_APPLE_HIGH_PERFORMANCE, ENCODING_RAW];
    expected.extend(APPLE_PSEUDO_ENCODINGS);
    expected.push(ENCODING_APPLE_MEDIA_STREAM);
    assert_eq!(
        twice, expected,
        "1010 leads and 1110 ends the list once, around the pixel and Apple encodings"
    );
}
