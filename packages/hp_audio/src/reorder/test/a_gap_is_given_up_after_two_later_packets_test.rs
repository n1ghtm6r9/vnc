use crate::AudioReorder;
use crate::test_support::{TEST_SSRC, frame_seqs, push_all};

#[test]
fn test_a_gap_is_given_up_after_two_later_packets() {
    let mut reorder = AudioReorder::default();

    let waiting = push_all(&mut reorder, TEST_SSRC, [10, 13, 14]);
    let released = push_all(&mut reorder, TEST_SSRC, [15]);

    assert_eq!(
        frame_seqs(&waiting),
        vec![(0, 10)],
        "two later packets may wait for the missing ones"
    );
    assert_eq!(
        frame_seqs(&released),
        vec![(2, 13), (0, 14), (0, 15)],
        "the third gives the two missing packets up as lost, before the 60 ms buffer runs dry"
    );
}
