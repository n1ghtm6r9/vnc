use std::mem::offset_of;

use crate::CuvidPicParams;

#[test]
fn test_the_picture_params_start_like_the_c_layout() {
    assert_eq!(
        offset_of!(CuvidPicParams, curr_pic_idx),
        8,
        "the picture index sits where CUVIDPICPARAMS keeps CurrPicIdx"
    );
}
