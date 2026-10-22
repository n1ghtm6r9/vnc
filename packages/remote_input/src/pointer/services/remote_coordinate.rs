pub(crate) fn remote_coordinate(
    window: f64,
    view_start: f64,
    view_length: f64,
    remote_length: u16,
) -> u16 {
    let last_pixel = f64::from(remote_length.saturating_sub(1));
    let last_view_pixel = view_length - 1.0;
    if last_view_pixel <= 0.0 {
        return 0;
    }
    let scaled = (window - view_start) * last_pixel / last_view_pixel;
    scaled.round().clamp(0.0, last_pixel) as u16
}
