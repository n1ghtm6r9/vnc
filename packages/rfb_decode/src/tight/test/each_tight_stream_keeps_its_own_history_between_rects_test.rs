use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, rgba_pixel};

#[test]
fn test_each_tight_stream_keeps_its_own_history_between_rects() {
    let mut harness = DecodeHarness::new(4, 3);
    let mut streams = [ZlibFixture::new(), ZlibFixture::new()];
    let pixels: Vec<u8> = (0..36u8).map(|index| index * 6 + 2).collect();
    let mut sizes = Vec::new();
    for (round, stream) in [0usize, 1, 0, 1].into_iter().enumerate() {
        let body = streams[stream].tight_payload(&pixels);
        sizes.push(body.len());
        let mut payload = vec![u8::try_from(stream).expect("a stream index fits a byte") << 4];
        payload.extend_from_slice(&body);
        harness
            .decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 3), &payload)
            .unwrap_or_else(|error| panic!("rect {round} on stream {stream} decodes: {error}"));
        assert_eq!(
            harness.pixel(1, 0),
            rgba_pixel(pixels[3], pixels[4], pixels[5]),
            "rect {round} holds the pixels"
        );
    }
    assert!(
        sizes[2] < sizes[0],
        "stream 0 reuses its history for the repeated rect"
    );
    assert!(
        sizes[3] < sizes[1],
        "stream 1 reuses its history for the repeated rect"
    );
}
