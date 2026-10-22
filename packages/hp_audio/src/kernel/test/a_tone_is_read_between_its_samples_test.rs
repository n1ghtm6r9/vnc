use crate::test_support::tone;
use crate::{KERNEL_CUTOFF, SincKernel};

#[test]
fn test_a_tone_is_read_between_its_samples() {
    let kernel = SincKernel::new(KERNEL_CUTOFF);
    let frames: Vec<[f32; 2]> = (0..256)
        .map(|index| {
            [
                tone(10_000.0, f64::from(index)),
                tone(1_000.0, f64::from(index)),
            ]
        })
        .collect();

    for phase in [0.1, 0.5, 0.77] {
        let at = 128.0 + phase;
        let [high, low] = kernel.interpolate(&frames, 128, phase);

        assert!(
            (high - tone(10_000.0, at)).abs() < 1e-3,
            "a 10 kHz tone reads {high} at {at}, not {}",
            tone(10_000.0, at)
        );
        assert!(
            (low - tone(1_000.0, at)).abs() < 1e-3,
            "a 1 kHz tone reads {low} at {at}, not {}",
            tone(1_000.0, at)
        );
    }
}
