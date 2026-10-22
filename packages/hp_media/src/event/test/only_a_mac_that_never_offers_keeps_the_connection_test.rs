use crate::MediaFailure;

#[test]
fn test_only_a_mac_that_never_offers_keeps_the_connection() {
    let others = [
        MediaFailure::BadMessage1("bad".into()),
        MediaFailure::Offer("no randomness".into()),
        MediaFailure::NoAnswer,
        MediaFailure::BadAnswer("bad".into()),
        MediaFailure::NoPicture,
        MediaFailure::Decoder("lost".into()),
        MediaFailure::Stalled,
        MediaFailure::MediaStopped,
        MediaFailure::UnsupportedVideo(96),
        MediaFailure::Socket("closed".into()),
    ];

    assert!(
        !MediaFailure::NoMessage1.needs_reconnect(),
        "a Mac that never offered stays in standard mode on the same connection"
    );
    for failure in others {
        assert!(
            failure.needs_reconnect(),
            "after \"{failure}\" the Mac needs a fresh connection in standard mode"
        );
    }
}
