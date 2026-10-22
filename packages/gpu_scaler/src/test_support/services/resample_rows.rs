use super::catmull_rom;
use crate::test_support::LinearImage;

pub fn resample_rows(image: &LinearImage, output_width: u32) -> LinearImage {
    let scale = f64::from(output_width) / f64::from(image.width);
    let support = (1.0 / scale).max(1.0);
    let last_column = i64::from(image.width) - 1;
    let mut pixels = Vec::with_capacity(output_width as usize * image.height as usize);
    for row in image.pixels.chunks_exact(image.width as usize) {
        for x in 0..output_width {
            let centre = (f64::from(x) + 0.5) / scale - 0.5;
            let first = (centre - 2.0 * support).floor() as i64 + 1;
            let last = (centre + 2.0 * support).floor() as i64;
            let mut sum = [0.0; 3];
            let mut weight_sum = 0.0;
            for tap in first..=last {
                let weight = catmull_rom((tap as f64 - centre) / support);
                let texel = row[tap.clamp(0, last_column) as usize];
                for (accumulator, value) in sum.iter_mut().zip(texel) {
                    *accumulator += value * weight;
                }
                weight_sum += weight;
            }
            pixels.push(sum.map(|value| value / weight_sum));
        }
    }
    LinearImage {
        width: output_width,
        height: image.height,
        pixels,
    }
}
