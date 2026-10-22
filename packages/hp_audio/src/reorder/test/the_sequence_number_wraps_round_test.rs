use crate::AudioReorder;
use crate::test_support::{TEST_SSRC, frame_seqs, push_all};

#[test]
fn test_the_sequence_number_wraps_round() {
    let mut reorder = AudioReorder::default();

    let frames = push_all(&mut reorder, TEST_SSRC, [65534, 65535, 0, 1]);

    assert_eq!(
        frame_seqs(&frames),
        vec![(0, 65534), (0, 65535), (0, 0), (0, 1)],
        "the count goes on past 65535"
    );
}
