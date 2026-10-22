use gpu_scaler::{YuvColour, YuvLayout, YuvStripe};
use rfb_session::{ChromaLayout, ColourTransform, DecodedFrame, PlaneBuffer, StripeImage};

use crate::yuv_stripe;

#[test]
fn test_a_picture_stripe_reaches_the_gpu_with_its_planes() {
    let colour = ColourTransform {
        rows: [[1.0, 0.0, 1.5, -0.75], [1.0, -0.2, -0.5, 0.35], [1.0, 1.8, 0.0, -0.9]],
        primaries: [[1.2, -0.2, 0.0], [0.0, 1.0, 0.0], [0.0, -0.1, 1.1]],
    };
    let gpu_colour = YuvColour { rows: colour.rows, primaries: colour.primaries };
    for (layout, gpu_layout, plane_bytes) in [(ChromaLayout::Yuv444, YuvLayout::Planar444, 24), (ChromaLayout::Nv12, YuvLayout::Nv12, 12)] {
        let planes: Vec<u8> = (0..plane_bytes).collect();
        let stripe = StripeImage {
            y: 544,
            width: 3,
            height: 1,
            frame: DecodedFrame { width: 4, height: 2, layout, colour, planes: PlaneBuffer::from(planes.clone()) },
        };

        assert_eq!(
            yuv_stripe(&stripe),
            YuvStripe { y: 544, width: 3, height: 1, plane_width: 4, plane_height: 2, layout: gpu_layout, colour: gpu_colour, planes: &planes },
            "a {layout:?} stripe keeps its place, its visible size, its whole planes and its colour transform for the GPU"
        );
    }
}
