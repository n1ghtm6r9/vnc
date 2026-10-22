use super::ColourTransform;

impl ColourTransform {
    pub fn encoded_rgb(&self, y: u8, u: u8, v: u8) -> [u8; 3] {
        let codes = [y, u, v].map(|code| f32::from(code) / 255.0);
        self.rows.map(|[luma, blue, red, offset]| {
            let value = luma * codes[0] + blue * codes[1] + red * codes[2] + offset;
            (value.clamp(0.0, 1.0) * 255.0).round() as u8
        })
    }
}
