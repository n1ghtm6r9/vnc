use crate::{ConvertParams, YuvColour, YuvLayout, YuvStripe};

#[test]
fn test_the_convert_params_match_the_shader_uniform_layout() {
    let stripe = YuvStripe {
        y: 7,
        width: 1,
        height: 1,
        plane_width: 1,
        plane_height: 1,
        layout: YuvLayout::Nv12,
        colour: YuvColour {
            rows: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
            ],
            primaries: [[13.0, 14.0, 15.0], [16.0, 17.0, 18.0], [19.0, 20.0, 21.0]],
        },
        planes: &[],
    };

    assert_eq!(
        size_of::<ConvertParams>(),
        112,
        "the uniform struct of the yuv shader takes 112 bytes"
    );
    let params = ConvertParams::new(&stripe);
    let words: [u32; 28] = bytemuck::cast(params);
    assert_eq!(
        words[..4],
        [0, 7, 1, 1],
        "the stripe origin, the chroma shift and the interleaved chroma flag come first"
    );
    let floats: [f32; 28] = bytemuck::cast(params);
    assert_eq!(
        floats[4..16],
        [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0
        ],
        "the conversion rows follow"
    );
    assert_eq!(
        floats[16..],
        [
            13.0, 14.0, 15.0, 0.0, 16.0, 17.0, 18.0, 0.0, 19.0, 20.0, 21.0, 0.0
        ],
        "the primaries follow, each row padded to four floats"
    );
    let planar = ConvertParams::new(&YuvStripe {
        layout: YuvLayout::Planar444,
        ..stripe
    });
    let words: [u32; 28] = bytemuck::cast(planar);
    assert_eq!(
        words[2..4],
        [0, 0],
        "planar chroma is neither shifted nor interleaved"
    );
}
