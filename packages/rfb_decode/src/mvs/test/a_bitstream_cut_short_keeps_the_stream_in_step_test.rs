use rfb::Rect;

use crate::test_support::{DecodeHarness, mvs_gray_full_update, rgba_pixel};

#[test]
fn test_a_bitstream_cut_short_keeps_the_stream_in_step() {
    let mut harness = DecodeHarness::new(16, 8);
    let rect = Rect::new(0, 0, 16, 8);
    let mut cut = mvs_gray_full_update();
    cut.truncate(cut.len() - 3);

    harness.decode_mvs(rect, &cut);
    let after_cut = harness.pixel(0, 0);
    harness.decode_mvs(rect, &mvs_gray_full_update());

    assert_eq!(
        after_cut,
        rgba_pixel(0, 0, 0),
        "a tile whose bits run out is not painted"
    );
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(148, 148, 148),
        "the next update decodes as if nothing had happened"
    );
}
