pub fn is_near(frame: [f32; 2], expected: [f32; 2]) -> bool {
    frame
        .iter()
        .zip(expected)
        .all(|(sample, expected)| (sample - expected).abs() < 1e-3)
}
