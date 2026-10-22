use std::time::Instant;

use crate::{
    MediaChannel,
    test_support::{
        FAKE_FRAME_HEIGHT, FIRST_DON, NAL_TRAIL, deliver, fake_unit, streaming_session,
        stripe_images,
    },
};

#[test]
fn test_stripes_fill_the_picture_from_the_top() {
    let start = Instant::now();
    let (mut session, mut server) = streaming_session(start);

    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON + 1, 1, &[&fake_unit(NAL_TRAIL, 2)]),
        start,
    );
    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON + 2, 0, &[&fake_unit(NAL_TRAIL, 3)]),
        start,
    );

    let placed: Vec<(u32, u8)> = stripe_images(&session.effects.take().events)
        .iter()
        .map(|image| (image.y, image.frame.planes[0]))
        .collect();
    assert_eq!(
        placed,
        [(FAKE_FRAME_HEIGHT, 2), (0, 3)],
        "the second stripe goes one stripe height down and the next first stripe back on top"
    );
}
