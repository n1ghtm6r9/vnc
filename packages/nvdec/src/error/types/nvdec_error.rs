use std::{error::Error, fmt};

#[derive(Debug)]
pub enum NvdecError {
    Load(libloading::Error),
    Cuda {
        call: &'static str,
        code: i32,
        name: String,
    },
    Unsupported(String),
    CallbackPanicked,
}

impl fmt::Display for NvdecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NvdecError::Load(error) => {
                write!(f, "the NVIDIA video decoder could not be loaded: {error}")
            }
            NvdecError::Cuda { call, code, name } => {
                write!(f, "{call} failed with {name} ({code})")
            }
            NvdecError::Unsupported(reason) => {
                write!(f, "the GPU cannot decode this video: {reason}")
            }
            NvdecError::CallbackPanicked => {
                write!(f, "the video decoder stopped on an internal error")
            }
        }
    }
}

impl Error for NvdecError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NvdecError::Load(error) => Some(error),
            _ => None,
        }
    }
}
