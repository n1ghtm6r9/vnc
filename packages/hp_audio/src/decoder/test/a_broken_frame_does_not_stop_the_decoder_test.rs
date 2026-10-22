use crate::test_support::SILENT_FRAME;
use crate::{EldDecoder, FRAME_SAMPLES};

#[test]
fn test_a_broken_frame_does_not_stop_the_decoder() {
    let mut decoder = EldDecoder::new().expect("the decoder takes the Mac's sound format");

    let _ = decoder.decode(&[0xff; 40]);
    let after: Vec<_> = (0..3).map(|_| decoder.decode(&SILENT_FRAME)).collect();

    assert!(
        after.iter().all(Result::is_ok),
        "the frames after a broken one still decode"
    );
    assert_eq!(
        after.last().cloned().and_then(Result::ok),
        Some(vec![[0.0, 0.0]; FRAME_SAMPLES]),
        "the silence after a broken frame is silent again"
    );
}
