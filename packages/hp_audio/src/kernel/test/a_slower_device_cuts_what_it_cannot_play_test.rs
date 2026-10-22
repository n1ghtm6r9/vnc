use crate::output_cutoff;

#[test]
fn test_a_slower_device_cuts_what_it_cannot_play() {
    assert!(
        (output_cutoff(48_000) - 0.45).abs() < 1e-9,
        "a 48 kHz device keeps the sound up to 0.45 of its sample rate"
    );
    assert!(
        (output_cutoff(44_100) - 0.45 * 44_100.0 / 48_000.0).abs() < 1e-9,
        "a 44.1 kHz device keeps only what it can play"
    );
    assert!(
        (output_cutoff(96_000) - 0.45).abs() < 1e-9,
        "a faster device keeps all of the sound"
    );
}
