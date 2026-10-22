use rfb::Rect;

use crate::test_support::{DecodeHarness, mvs_luma_ac_full_update, mvs_luma_step_partial_update};

#[test]
fn test_the_luma_baseline_of_a_partial_update_comes_from_the_full_update() {
    let rect = Rect::new(0, 0, 16, 8);
    let tile = Rect::new(0, 0, 8, 8);
    let mut full = DecodeHarness::new(16, 8);
    full.decode_mvs(rect, &mvs_luma_ac_full_update());
    let mut stepped = DecodeHarness::new(16, 8);
    stepped.decode_mvs(rect, &mvs_luma_ac_full_update());
    stepped.decode_mvs(rect, &mvs_luma_step_partial_update("1"));
    let mut restored = DecodeHarness::new(16, 8);
    restored.decode_mvs(rect, &mvs_luma_ac_full_update());
    restored.decode_mvs(rect, &mvs_luma_step_partial_update("1"));
    restored.decode_mvs(rect, &mvs_luma_step_partial_update("0"));

    assert_ne!(
        stepped.framebuffer.read_rect(tile),
        full.framebuffer.read_rect(tile),
        "a luma AC step changes the tile"
    );
    assert_eq!(
        restored.framebuffer.read_rect(tile),
        full.framebuffer.read_rect(tile),
        "no step from the full update gives the full update's tile back"
    );
}
