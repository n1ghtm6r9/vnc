use std::time::Instant;

use crate::{MediaEvent, test_support::streaming_session};

#[test]
fn test_a_new_display_goes_out_before_a_fresh_offer() {
    let start = Instant::now();
    let (mut session, _server) = streaming_session(start);
    let display = vec![0x1d, 0, 0, 1, 7];

    session.resize(display.clone(), start);
    let events = session.effects.take().events;

    assert!(
        matches!(&events[..], [MediaEvent::Display(sent), MediaEvent::Offer(_)] if *sent == display),
        "the Mac gets the new display first and a fresh offer for its stream right after, got {events:?}"
    );
}
