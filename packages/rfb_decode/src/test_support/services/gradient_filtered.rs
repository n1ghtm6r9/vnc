pub fn gradient_filtered(width: usize, rgb: &[u8]) -> Vec<u8> {
    let row_bytes = width * 3;
    let value = |x: isize, y: isize, component: usize| -> i32 {
        if x < 0 || y < 0 {
            return 0;
        }
        i32::from(rgb[y as usize * row_bytes + x as usize * 3 + component])
    };
    let mut filtered = Vec::with_capacity(rgb.len());
    for (index, &actual) in rgb.iter().enumerate() {
        let (x, y, component) = (
            (index % row_bytes / 3) as isize,
            (index / row_bytes) as isize,
            index % 3,
        );
        let predicted = (value(x - 1, y, component) + value(x, y - 1, component)
            - value(x - 1, y - 1, component))
        .clamp(0, 255);
        filtered.push((i32::from(actual) - predicted).rem_euclid(256) as u8);
    }
    filtered
}
