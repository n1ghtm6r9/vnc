pub fn pixel_is_close_to(pixel: u32, expected: [u8; 3], tolerance: u8) -> bool {
    let [red, green, blue, alpha] = pixel.to_le_bytes();
    alpha == 255
        && [red, green, blue]
            .iter()
            .zip(expected)
            .all(|(&actual, wanted)| actual.abs_diff(wanted) <= tolerance)
}
