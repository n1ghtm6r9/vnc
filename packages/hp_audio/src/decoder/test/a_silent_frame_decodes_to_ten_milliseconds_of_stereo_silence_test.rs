use crate::test_support::SILENT_FRAME;
use crate::{EldDecoder, FRAME_SAMPLES};

#[test]
fn test_a_silent_frame_decodes_to_ten_milliseconds_of_stereo_silence() {
    let mut decoder = EldDecoder::new().expect("the decoder takes the Mac's sound format");

    let sound = decoder
        .decode(&SILENT_FRAME)
        .expect("the Mac's silent frame decodes");

    assert_eq!(
        sound,
        vec![[0.0, 0.0]; FRAME_SAMPLES],
        "a silent frame is 480 stereo samples of silence"
    );
}
