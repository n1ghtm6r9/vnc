pub(crate) fn rgbx32_row_to_rgba(src: &[u8], dst: &mut [u32]) {
    let (pixels, _) = src.as_chunks::<4>();
    for (pixel, &[red, green, blue, _]) in dst.iter_mut().zip(pixels) {
        *pixel = u32::from_le_bytes([red, green, blue, 0xFF]);
    }
}
