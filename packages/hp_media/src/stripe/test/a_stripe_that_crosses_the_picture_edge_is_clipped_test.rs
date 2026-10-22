use crate::{StripeImage, stripe_image, test_support::fake_frame};

#[test]
fn test_a_stripe_that_crosses_the_picture_edge_is_clipped() {
    let image = stripe_image(fake_frame(4, 3, 5), 1, 3, 5);

    assert_eq!(
        image,
        Some(StripeImage {
            y: 3,
            width: 3,
            height: 2,
            frame: fake_frame(4, 3, 5),
        }),
        "only the rows and columns inside the picture are shown, from the whole frame as decoded"
    );
}
