use std::sync::Arc;

use log::warn;
use winit::window::Window;

use super::{GpuContext, GpuError};
use crate::preferred_backends;

impl GpuContext {
    pub fn new(window: Arc<Window>) -> Result<GpuContext, GpuError> {
        let preferred = preferred_backends();
        match GpuContext::open(window.clone(), preferred) {
            Err(error) if preferred != wgpu::Backends::all() => {
                warn!("[GPU] {error}; trying every backend");
                GpuContext::open(window, wgpu::Backends::all())
            }
            result => result,
        }
    }
}
