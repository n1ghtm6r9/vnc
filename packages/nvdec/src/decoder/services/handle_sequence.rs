use std::{
    ffi::{c_int, c_void},
    panic::{self, AssertUnwindSafe},
};

use crate::{CuvidVideoFormat, DecoderState};

pub(crate) unsafe extern "system" fn handle_sequence(
    user_data: *mut c_void,
    video: *mut CuvidVideoFormat,
) -> c_int {
    let state = unsafe { &mut *user_data.cast::<DecoderState>() };
    let video = unsafe { &*video };
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| state.start_sequence(video)));
    state.settle(outcome)
}
