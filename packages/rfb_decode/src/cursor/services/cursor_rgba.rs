use crate::PixelConverter;

pub(crate) fn cursor_rgba(
    converter: &PixelConverter,
    data: &[u8],
    width: usize,
    height: usize,
) -> Vec<u8> {
    let pixel_row_bytes = width * converter.bytes_per_pixel();
    let mask_row_bytes = width.div_ceil(8);
    let Some((pixels, mask)) = data.split_at_checked(pixel_row_bytes * height) else {
        return Vec::new();
    };
    let mut rgba = Vec::with_capacity(width * height * 4);
    let mut row = vec![0u32; width];
    for (pixel_row, mask_row) in pixels
        .chunks_exact(pixel_row_bytes)
        .zip(mask.chunks_exact(mask_row_bytes))
    {
        converter.rgba_row(pixel_row, &mut row);
        for (column, colour) in row.iter().enumerate() {
            let visible = (mask_row[column / 8] >> (7 - column % 8)) & 1 == 1;
            rgba.extend_from_slice(&if visible {
                colour.to_le_bytes()
            } else {
                [0; 4]
            });
        }
    }
    rgba
}
