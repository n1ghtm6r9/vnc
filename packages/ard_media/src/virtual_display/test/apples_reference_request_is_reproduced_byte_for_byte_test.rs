use crate::{DisplayMode, VirtualDisplay, test_support::hex_bytes, virtual_display_message};

#[test]
fn test_apples_reference_request_is_reproduced_byte_for_byte() {
    let mode = |pixel_width, pixel_height, width, height| DisplayMode {
        pixel_width,
        pixel_height,
        width,
        height,
        refresh: 60.0,
    };
    let apple = VirtualDisplay {
        name: "Screen Sharing Virtual Display".into(),
        largest_pixel_width: 0,
        largest_pixel_height: 0,
        modes: vec![
            mode(3840, 2160, 1920, 1080),
            mode(2880, 1800, 1440, 900),
            mode(3840, 2160, 1920, 1080),
            mode(2880, 1620, 1440, 810),
            mode(2624, 1696, 1312, 848),
        ],
    };
    let mut reference = hex_bytes(
        "1d0001300001000100000000012853637265656e2053686172696e67205669727475616c20446973706c6179",
    );
    reference.extend_from_slice(&[0; 98]);
    reference.extend(hex_bytes(
        "43b8ba2f434fd17400000f000000087000000000000000070005\
         00000f00000008700000078000000438404e00000000000000000000\
         00000b4000000708000005a000000384404e00000000000000000000\
         00000f00000008700000078000000438404e00000000000000000000\
         00000b4000000654000005a00000032a404e00000000000000000000\
         00000a40000006a00000052000000350404e00000000000000000000",
    ));

    let message = virtual_display_message(&[apple]);

    assert_eq!(
        message.len(),
        308,
        "the message for five modes is 308 bytes"
    );
    assert_eq!(
        message, reference,
        "the message matches what Apple's viewer sends"
    );
}
