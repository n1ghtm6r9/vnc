#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YuvColour {
    pub rows: [[f32; 4]; 3],
    pub primaries: [[f32; 3]; 3],
}
