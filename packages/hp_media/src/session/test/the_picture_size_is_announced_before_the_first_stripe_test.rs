use std::time::Instant;

use crate::{
    MediaChannel, MediaEvent, MediaState,
    test_support::{
        FIRST_DON, FakeDecoder, NAL_IDR, TestServer, answered_session, deliver, fake_unit,
    },
};

#[test]
fn test_the_picture_size_is_announced_before_the_first_stripe() {
    let start = Instant::now();
    let mut session = answered_session(FakeDecoder::default(), start);
    let mut server = TestServer::new();

    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 7)]),
        start,
    );
    let order: Vec<&str> = session
        .effects
        .take()
        .events
        .iter()
        .filter_map(|event| match event {
            MediaEvent::State(MediaState::Streaming { .. }) => Some("streaming"),
            MediaEvent::Stripe(_) => Some("stripe"),
            _ => None,
        })
        .collect();

    assert_eq!(
        order,
        ["streaming", "stripe"],
        "the app learns the picture size before the first stripe has to fit into it"
    );
}
