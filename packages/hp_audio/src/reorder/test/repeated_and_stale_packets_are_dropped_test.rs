use crate::AudioReorder;
use crate::test_support::{TEST_SSRC, frame_seqs, push_all};

#[test]
fn test_repeated_and_stale_packets_are_dropped() {
    let mut reorder = AudioReorder::default();

    let frames = push_all(&mut reorder, TEST_SSRC, [10, 12, 12, 11, 11, 10, 13]);

    assert_eq!(
        frame_seqs(&frames),
        vec![(0, 10), (0, 11), (0, 12), (0, 13)],
        "a packet that is already waiting or already played is not played again"
    );
}
