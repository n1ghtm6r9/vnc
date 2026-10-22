use winit::dpi::PhysicalSize;
use winit::platform::windows::IconExtWindows;
use winit::window::Icon;

use crate::APP_ICON_RESOURCE;

pub fn app_icon(side: u32) -> Option<Icon> {
    Icon::from_resource(APP_ICON_RESOURCE, Some(PhysicalSize::new(side, side))).ok()
}
