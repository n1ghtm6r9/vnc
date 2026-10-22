use super::{ColourMatrix, ColourPrimaries, ColourTransform};

impl ColourTransform {
    pub(crate) fn new(matrix: ColourMatrix, full_range: bool, primaries: ColourPrimaries) -> Self {
        let (luma_scale, chroma_scale, black) = if full_range {
            (1.0, 1.0, 0.0)
        } else {
            (255.0 / 219.0, 255.0 / 224.0, 16.0)
        };
        let (weights, codes) = match matrix.luma_weights() {
            None => (
                [
                    [0.0, 0.0, luma_scale],
                    [luma_scale, 0.0, 0.0],
                    [0.0, luma_scale, 0.0],
                ],
                [black; 3],
            ),
            Some((kr, kb)) => {
                let kg = 1.0 - kr - kb;
                (
                    [
                        [luma_scale, 0.0, chroma_scale * 2.0 * (1.0 - kr)],
                        [
                            luma_scale,
                            -chroma_scale * 2.0 * kb * (1.0 - kb) / kg,
                            -chroma_scale * 2.0 * kr * (1.0 - kr) / kg,
                        ],
                        [luma_scale, chroma_scale * 2.0 * (1.0 - kb), 0.0],
                    ],
                    [black, 128.0, 128.0],
                )
            }
        };
        let rows = weights.map(|[luma, blue, red]: [f64; 3]| {
            let offset = -(luma * codes[0] + blue * codes[1] + red * codes[2]) / 255.0;
            [luma as f32, blue as f32, red as f32, offset as f32]
        });
        Self {
            rows,
            primaries: primaries.to_srgb(),
        }
    }
}
