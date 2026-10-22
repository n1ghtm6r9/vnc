use crate::EldDecoder;

#[test]
fn test_an_empty_frame_is_refused() {
    let mut decoder = EldDecoder::new().expect("the decoder takes the Mac's sound format");

    let sound = decoder.decode(&[]);

    assert!(
        sound.is_err(),
        "a packet with no sound in it is not a frame"
    );
}
