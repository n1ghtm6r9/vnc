use crate::{stripe_image, test_support::fake_frame};

#[test]
fn test_a_stripe_beyond_the_picture_is_dropped() {
    assert_eq!(
        stripe_image(fake_frame(4, 3, 0), 3, 4, 9),
        None,
        "a stripe that starts at the bottom edge is not shown"
    );
    assert_eq!(
        stripe_image(fake_frame(4, 3, 0), u32::MAX, 4, 9),
        None,
        "a stripe number too large to place is not shown"
    );
}
