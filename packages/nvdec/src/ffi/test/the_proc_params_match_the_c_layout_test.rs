use std::mem::{offset_of, size_of};

use crate::CuvidProcParams;

#[test]
fn test_the_proc_params_match_the_c_layout() {
    let fields = [
        (
            "progressive_frame",
            offset_of!(CuvidProcParams, progressive_frame),
            0,
        ),
        ("second_field", offset_of!(CuvidProcParams, second_field), 4),
        (
            "top_field_first",
            offset_of!(CuvidProcParams, top_field_first),
            8,
        ),
        (
            "unpaired_field",
            offset_of!(CuvidProcParams, unpaired_field),
            12,
        ),
        (
            "reserved_flags",
            offset_of!(CuvidProcParams, reserved_flags),
            16,
        ),
        (
            "reserved_zero",
            offset_of!(CuvidProcParams, reserved_zero),
            20,
        ),
        (
            "raw_input_dptr",
            offset_of!(CuvidProcParams, raw_input_dptr),
            24,
        ),
        (
            "raw_input_pitch",
            offset_of!(CuvidProcParams, raw_input_pitch),
            32,
        ),
        (
            "raw_input_format",
            offset_of!(CuvidProcParams, raw_input_format),
            36,
        ),
        (
            "raw_output_dptr",
            offset_of!(CuvidProcParams, raw_output_dptr),
            40,
        ),
        (
            "raw_output_pitch",
            offset_of!(CuvidProcParams, raw_output_pitch),
            48,
        ),
        ("reserved1", offset_of!(CuvidProcParams, reserved1), 52),
        (
            "output_stream",
            offset_of!(CuvidProcParams, output_stream),
            56,
        ),
        ("reserved", offset_of!(CuvidProcParams, reserved), 64),
        (
            "histogram_dptr",
            offset_of!(CuvidProcParams, histogram_dptr),
            248,
        ),
        ("proc_ext", offset_of!(CuvidProcParams, proc_ext), 256),
    ];

    for (name, actual, expected) in fields {
        assert_eq!(
            actual, expected,
            "{name} sits where CUVIDPROCPARAMS keeps it"
        );
    }
    assert_eq!(
        size_of::<CuvidProcParams>(),
        264,
        "the proc params are as big as CUVIDPROCPARAMS"
    );
}
