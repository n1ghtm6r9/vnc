pub(crate) fn chroma_offsets(blue: u8, red: u8) -> [i32; 3] {
    let blue = i32::from(blue) - 128;
    let red = i32::from(red) - 128;
    [
        (91_881 * red + 32_768) >> 16,
        (32_768 - 22_554 * blue - 46_802 * red) >> 16,
        (116_130 * blue + 32_768) >> 16,
    ]
}
