use std::sync::{MutexGuard, PoisonError};

use crate::{ACTIVE_GRAB, GrabState};

pub(crate) fn lock_active_grab() -> MutexGuard<'static, Option<GrabState>> {
    ACTIVE_GRAB.lock().unwrap_or_else(PoisonError::into_inner)
}
