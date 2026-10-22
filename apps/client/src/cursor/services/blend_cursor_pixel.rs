use rfb_session::CursorShape;

pub fn blend_cursor_pixel(shape: &CursorShape, rows: &[(usize, f32)], columns: &[(usize, f32)]) -> [u8; 4] {
    let width = usize::from(shape.width);
    let mut sum = [0.0f32; 4];
    for &(y, row_weight) in rows {
        for &(x, column_weight) in columns {
            let offset = (y * width + x) * 4;
            let Some(&[red, green, blue, opacity]) = shape.rgba.get(offset..offset + 4) else {
                continue;
            };
            let weight = row_weight * column_weight * f32::from(opacity);
            sum[0] += f32::from(red) * weight;
            sum[1] += f32::from(green) * weight;
            sum[2] += f32::from(blue) * weight;
            sum[3] += weight;
        }
    }
    let alpha = sum[3];
    if alpha <= 0.0 {
        return [0; 4];
    }
    let byte = |value: f32| value.round().clamp(0.0, 255.0) as u8;
    [byte(sum[0] / alpha), byte(sum[1] / alpha), byte(sum[2] / alpha), byte(alpha)]
}
