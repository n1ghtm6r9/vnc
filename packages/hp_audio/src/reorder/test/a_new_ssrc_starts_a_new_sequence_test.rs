use crate::AudioReorder;
use crate::test_support::{frame_seqs, push_all};

#[test]
fn test_a_new_ssrc_starts_a_new_sequence() {
    let mut reorder = AudioReorder::default();

    let mut frames = push_all(&mut reorder, 1, [500, 501]);
    frames.extend(push_all(&mut reorder, 2, [7, 8]));

    assert_eq!(
        frame_seqs(&frames),
        vec![(0, 500), (0, 501), (0, 7), (0, 8)],
        "a restarted stream counts from its own first packet"
    );
}
