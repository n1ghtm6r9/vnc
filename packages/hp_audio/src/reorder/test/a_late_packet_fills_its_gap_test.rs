use crate::AudioReorder;
use crate::test_support::{TEST_SSRC, frame_seqs, push_all};

#[test]
fn test_a_late_packet_fills_its_gap() {
    let mut reorder = AudioReorder::default();

    let early = push_all(&mut reorder, TEST_SSRC, [10, 12]);
    let late = push_all(&mut reorder, TEST_SSRC, [11]);

    assert_eq!(
        frame_seqs(&early),
        vec![(0, 10)],
        "a packet after a gap waits for the missing one"
    );
    assert_eq!(
        frame_seqs(&late),
        vec![(0, 11), (0, 12)],
        "the late packet fills the gap and lets the waiting one go"
    );
}
