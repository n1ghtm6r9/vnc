use crate::apple_input_source;

#[test]
fn test_the_mac_input_source_is_read_from_the_keyboard_info() {
    let name = b"com.apple.keylayout.Russian";
    let mut payload = vec![0, 2, 0, 0, 0, 1, 0, 27];
    payload.extend_from_slice(name);
    payload.extend_from_slice(&[9, 9]);
    assert_eq!(
        apple_input_source(&payload).as_deref(),
        Some("com.apple.keylayout.Russian"),
        "the input source name follows the source count, the flags and its own length"
    );
}
