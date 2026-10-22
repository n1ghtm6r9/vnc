use std::sync::Mutex;

use crate::GrabState;

pub(crate) static ACTIVE_GRAB: Mutex<Option<GrabState>> = Mutex::new(None);
