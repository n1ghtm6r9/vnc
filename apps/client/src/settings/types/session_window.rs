use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionWindow {
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
    pub fullscreen: bool,
}
