#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SurfacePlane {
    pub(crate) index: usize,
    pub(crate) x_bytes: usize,
    pub(crate) y: usize,
    pub(crate) width_bytes: usize,
    pub(crate) rows: usize,
}
