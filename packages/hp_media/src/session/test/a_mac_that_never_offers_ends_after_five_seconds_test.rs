use std::time::{Duration, Instant};

use crate::{
    MESSAGE1_TIMEOUT, MediaFailure, MediaState,
    test_support::{FakeDecoder, states, test_session},
};

#[test]
fn test_a_mac_that_never_offers_ends_after_five_seconds() {
    let start = Instant::now();
    let mut session = test_session(FakeDecoder::default(), start);

    session.tick(start + MESSAGE1_TIMEOUT - Duration::from_millis(1));
    assert!(!session.is_finished(), "the Mac has five seconds to offer");

    session.tick(start + MESSAGE1_TIMEOUT);
    assert_eq!(
        states(&session.effects.take().events),
        [MediaState::Failed(MediaFailure::NoMessage1)],
        "a Mac that never offers ends High Performance"
    );
    assert!(session.is_finished(), "nothing more happens after that");
}
