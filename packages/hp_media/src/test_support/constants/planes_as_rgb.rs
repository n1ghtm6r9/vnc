use nvdec::ColourTransform;

pub const PLANES_AS_RGB: ColourTransform = ColourTransform {
    rows: [
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
    ],
    primaries: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
};
