use crate::{PLAYOUT_TARGET, PlayoutBuffer, SAMPLE_RATE};

#[test]
fn test_stereo_reaches_every_kind_of_device() {
    let mut mono = PlayoutBuffer::new(SAMPLE_RATE);
    let mut surround = PlayoutBuffer::new(SAMPLE_RATE);
    mono.push(&vec![[0.5, -0.25]; PLAYOUT_TARGET]);
    surround.push(&vec![[0.5, -0.25]; PLAYOUT_TARGET]);
    let mut one = vec![0.0; 480];
    let mut six = vec![0.0; 6 * 480];

    mono.render(&mut one, 1);
    surround.render(&mut six, 6);

    assert!(
        one[64..].iter().all(|sample| (sample - 0.125).abs() < 1e-3),
        "a mono device gets the middle of left and right"
    );
    assert!(
        six[6 * 64..].chunks(6).all(|frame| {
            (frame[0] - 0.5).abs() < 1e-3
                && (frame[1] + 0.25).abs() < 1e-3
                && frame[2..].iter().all(|sample| *sample == 0.0)
        }),
        "a surround device gets left and right on its front pair and silence elsewhere"
    );
}
