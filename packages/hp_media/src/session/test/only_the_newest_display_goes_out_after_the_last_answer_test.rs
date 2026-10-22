use std::time::Instant;

use crate::{
    MediaEvent,
    test_support::{streaming_session, test_answer},
};

#[test]
fn test_only_the_newest_display_goes_out_after_the_last_answer() {
    let start = Instant::now();
    let (mut session, _server) = streaming_session(start);
    session.resize(vec![0x1d, 1], start);
    session.effects.take();

    session.resize(vec![0x1d, 2], start);
    session.resize(vec![0x1d, 3], start);
    let unanswered = session.effects.take().events;
    session.handle_answer(&test_answer(), start);
    session.tick(start);
    let answered = session.effects.take().events;

    assert!(
        unanswered.is_empty(),
        "no display goes out while the Mac has not answered the last one, got {unanswered:?}"
    );
    assert!(
        matches!(&answered[..], [MediaEvent::Display(sent), MediaEvent::Offer(_)] if *sent == [0x1d, 3]),
        "once the Mac answers, the newest display goes out with its offer, got {answered:?}"
    );
}
