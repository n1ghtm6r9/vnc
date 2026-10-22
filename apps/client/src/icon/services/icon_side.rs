pub fn icon_side(side: u32, scale: f64) -> u32 {
    if !scale.is_finite() || scale < 1.0 {
        return side;
    }
    (f64::from(side) * scale).round() as u32
}
