use rfb::{ENCODING_RRE, Rect};

use crate::test_support::{DecodeHarness, rgba_pixel, rre_payload, rre_subrect, wire_pixel};

#[test]
fn test_rre_with_more_subrects_than_one_batch_is_read_completely() {
    let mut harness = DecodeHarness::new(50, 30);
    let subrects: Vec<Vec<u8>> = (0..1300u16)
        .map(|index| {
            rre_subrect(
                wire_pixel((index % 256) as u8, 1, 2),
                Rect::new(index % 50, index / 50, 1, 1),
            )
        })
        .collect();
    let payload = rre_payload(wire_pixel(0, 0, 0), &subrects);
    harness
        .decode(ENCODING_RRE, Rect::new(0, 0, 50, 30), &payload)
        .expect("a long RRE rect decodes");
    for index in [0u16, 1, 511, 512, 513, 1023, 1024, 1299] {
        let expected = rgba_pixel((index % 256) as u8, 1, 2);
        assert_eq!(
            harness.pixel(index % 50, index / 50),
            expected,
            "subrect {index} is painted whichever batch it fell into"
        );
    }
}
