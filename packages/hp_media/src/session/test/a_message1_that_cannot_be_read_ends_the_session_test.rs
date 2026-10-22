use std::time::Instant;

use crate::{
    MediaFailure, MediaState,
    test_support::{FakeDecoder, offer_count, states, test_session},
};

#[test]
fn test_a_message1_that_cannot_be_read_ends_the_session() {
    let start = Instant::now();
    let mut session = test_session(FakeDecoder::default(), start);

    session.handle_message1(&[0, 1, 2], start);
    let effects = session.effects.take();

    assert_eq!(
        offer_count(&effects.events),
        0,
        "nothing is offered for a broken Message 1"
    );
    assert!(
        matches!(
            states(&effects.events)[..],
            [MediaState::Failed(MediaFailure::BadMessage1(_))]
        ),
        "a broken Message 1 ends High Performance, got {:?}",
        states(&effects.events)
    );
}
