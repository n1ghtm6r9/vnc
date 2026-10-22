use std::time::Instant;

use crate::{
    MediaChannel, MediaEvent,
    test_support::{
        FIRST_DON, FakeDecoder, NAL_IDR, TestServer, answered_session, deliver, fake_unit,
    },
};

#[test]
fn test_a_resize_before_the_first_picture_waits_for_it() {
    let start = Instant::now();
    let mut session = answered_session(FakeDecoder::default(), start);
    let mut server = TestServer::new();

    session.resize(vec![0x1d, 1], start);
    let early = session.effects.take().events;
    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 1)]),
        start,
    );
    session.tick(start);
    let shown = session.effects.take().events;

    assert!(
        early.is_empty(),
        "no display goes out before the first stream shows a picture, got {early:?}"
    );
    assert!(
        matches!(&shown[..], [.., MediaEvent::Display(sent), MediaEvent::Offer(_)] if *sent == [0x1d, 1]),
        "the display goes out with its offer once the first picture is shown, got {shown:?}"
    );
}
