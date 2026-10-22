use crate::{
    ColourMatrix, ColourPrimaries, ColourTransform,
    test_support::{BLUE, RED, assert_rgb_near},
};

#[test]
fn test_limited_bt709_turns_studio_red_and_blue_pure() {
    let transform = ColourTransform::new(ColourMatrix::Bt709, false, ColourPrimaries::Bt709);

    assert_rgb_near(transform.encoded_rgb(63, 102, 240), RED, "studio red");
    assert_rgb_near(transform.encoded_rgb(32, 240, 118), BLUE, "studio blue");
}
