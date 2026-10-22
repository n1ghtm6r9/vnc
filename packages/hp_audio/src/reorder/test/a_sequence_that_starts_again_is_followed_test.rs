use crate::AudioReorder;
use crate::test_support::{TEST_SSRC, frame_seqs, push_all};

#[test]
fn test_a_sequence_that_starts_again_is_followed() {
    let mut reorder = AudioReorder::default();

    let frames = push_all(&mut reorder, TEST_SSRC, [5000].into_iter().chain(100..=126));

    assert_eq!(
        frame_seqs(&frames),
        vec![(0, 5000), (0, 125), (0, 126)],
        "after a quarter of a second of packets from the past, the stream follows them"
    );
}
