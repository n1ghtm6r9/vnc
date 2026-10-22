use crate::{KERNEL_CUTOFF, SincKernel};

#[test]
fn test_a_steady_level_stays_steady_between_samples() {
    let kernel = SincKernel::new(KERNEL_CUTOFF);
    let frames = vec![[0.5, -0.25]; 128];

    for phase in [0.0, 0.25, 0.5, 0.999] {
        let [left, right] = kernel.interpolate(&frames, 64, phase);

        assert!(
            (left - 0.5).abs() < 1e-5 && (right + 0.25).abs() < 1e-5,
            "a steady level reads the same {phase} of the way to the next sample, not {left} and {right}"
        );
    }
}
