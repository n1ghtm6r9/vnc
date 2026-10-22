pub fn catmull_rom(x: f64) -> f64 {
    let distance = x.abs();
    if distance < 1.0 {
        1.5 * distance.powi(3) - 2.5 * distance.powi(2) + 1.0
    } else if distance < 2.0 {
        -0.5 * distance.powi(3) + 2.5 * distance.powi(2) - 4.0 * distance + 2.0
    } else {
        0.0
    }
}
