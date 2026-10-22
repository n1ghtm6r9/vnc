#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColourTransform {
    pub rows: [[f32; 4]; 3],
    pub primaries: [[f32; 3]; 3],
}
