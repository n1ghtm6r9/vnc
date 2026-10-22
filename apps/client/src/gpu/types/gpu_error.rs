use std::fmt;

#[derive(Debug)]
pub enum GpuError {
    Surface(String),
    Adapter(String),
    Device(String),
}

impl fmt::Display for GpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GpuError::Surface(reason) => write!(f, "the window surface could not be created: {reason}"),
            GpuError::Adapter(reason) => write!(f, "no graphics adapter can draw to this window: {reason}"),
            GpuError::Device(reason) => write!(f, "the graphics device could not be opened: {reason}"),
        }
    }
}

impl std::error::Error for GpuError {}
