use crate::apple_input_source;

#[test]
fn test_a_broken_keyboard_info_has_no_input_source() {
    let payloads: [&[u8]; 3] = [
        &[0, 1, 0, 0],
        &[0, 1, 0, 0, 0, 0, 0, 9, b'a'],
        &[0, 1, 0, 0, 0, 0, 0, 2, 0xff, 0xfe],
    ];
    for payload in payloads {
        assert_eq!(
            apple_input_source(payload),
            None,
            "a cut header, a cut name or a name that is not UTF-8 gives no input source"
        );
    }
}
