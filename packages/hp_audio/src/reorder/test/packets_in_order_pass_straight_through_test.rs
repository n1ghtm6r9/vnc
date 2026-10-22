use crate::AudioReorder;
use crate::test_support::{TEST_SSRC, frame_seqs, push_all};

#[test]
fn test_packets_in_order_pass_straight_through() {
    let mut reorder = AudioReorder::default();

    let frames = push_all(&mut reorder, TEST_SSRC, [100, 101, 102]);

    assert_eq!(
        frame_seqs(&frames),
        vec![(0, 100), (0, 101), (0, 102)],
        "each packet that comes in order is a frame at once, with nothing lost"
    );
}
