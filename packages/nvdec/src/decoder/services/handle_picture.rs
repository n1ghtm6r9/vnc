use std::{
    ffi::{c_int, c_void},
    panic::{self, AssertUnwindSafe},
};

use crate::{CuvidPicParams, DecoderState};

pub(crate) unsafe extern "system" fn handle_picture(
    user_data: *mut c_void,
    picture: *mut CuvidPicParams,
) -> c_int {
    let state = unsafe { &mut *user_data.cast::<DecoderState>() };
    let index = unsafe { (*picture).curr_pic_idx };
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| state.decode_picture(picture, index)));
    state.settle(outcome)
}
