pub fn pan_offset(window_length: u32, remote_length: u16, pointer: Option<f64>) -> f64 {
    let window_length = f64::from(window_length);
    let remote_length = f64::from(remote_length);
    if remote_length <= window_length {
        return ((window_length - remote_length) / 2.0).floor();
    }
    let fraction = pointer
        .map(|position| (position / window_length).clamp(0.0, 1.0))
        .unwrap_or(0.5);
    -((remote_length - window_length) * fraction).round()
}
