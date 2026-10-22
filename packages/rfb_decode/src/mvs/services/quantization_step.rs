pub(crate) fn quantization_step(position: usize, threshold: u8) -> i32 {
    let threshold = usize::from(threshold);
    let fine = threshold > 14;
    let below = position < threshold;
    if fine && (position <= 5 || below) {
        2
    } else if fine || below {
        8
    } else {
        16
    }
}
