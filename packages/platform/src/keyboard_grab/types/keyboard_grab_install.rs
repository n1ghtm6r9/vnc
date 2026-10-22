use std::sync::Arc;

use super::{GrabState, GrabbedKey, KeyboardGrab};
use crate::{PlatformError, lock_active_grab};

impl KeyboardGrab {
    pub fn install(window: isize, on_key: Box<dyn Fn(GrabbedKey) + Send + Sync>) -> Result<KeyboardGrab, PlatformError> {
        {
            let mut slot = lock_active_grab();
            if slot.is_some() {
                return Err(PlatformError::Os("a keyboard grab is already installed".to_string()));
            }
            *slot = Some(GrabState {
                window,
                active: false,
                on_key: Arc::from(on_key),
                swallowed: Vec::new(),
            });
        }
        match KeyboardGrab::hook() {
            Ok(hook) => Ok(KeyboardGrab { hook }),
            Err(error) => {
                *lock_active_grab() = None;
                Err(error)
            }
        }
    }
}
