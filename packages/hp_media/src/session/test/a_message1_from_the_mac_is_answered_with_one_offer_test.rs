use std::time::Instant;

use crate::{
    MediaEvent,
    test_support::{FakeDecoder, message1_payload, offer_count, test_session},
};

#[test]
fn test_a_message1_from_the_mac_is_answered_with_one_offer() {
    let start = Instant::now();
    let mut session = test_session(FakeDecoder::default(), start);

    session.handle_message1(&message1_payload(), start);
    session.handle_message1(&message1_payload(), start);
    let effects = session.effects.take();

    assert_eq!(
        offer_count(&effects.events),
        1,
        "one offer goes out, however often the Mac asks"
    );
    let Some(MediaEvent::Offer(offer)) = effects.events.first() else {
        panic!("the offer is the first thing the session sends");
    };
    assert_eq!(
        offer[0], 0x1c,
        "the offer is a media stream negotiation message"
    );
    assert!(
        !session.is_finished(),
        "the session waits for the Mac's answer"
    );
}
