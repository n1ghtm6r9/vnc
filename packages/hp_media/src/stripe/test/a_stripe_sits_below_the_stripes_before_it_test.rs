use crate::{StripeImage, stripe_image, test_support::fake_frame};

#[test]
fn test_a_stripe_sits_below_the_stripes_before_it() {
    let image = stripe_image(fake_frame(4, 3, 5), 2, 4, 9);

    assert_eq!(
        image,
        Some(StripeImage {
            y: 6,
            width: 4,
            height: 3,
            frame: fake_frame(4, 3, 5),
        }),
        "the third stripe starts two stripe heights down and shows its whole frame"
    );
}
