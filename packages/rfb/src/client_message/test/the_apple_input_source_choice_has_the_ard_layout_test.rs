use crate::{ClientMessage, write_apple_set_input_source};

#[test]
fn test_the_apple_input_source_choice_has_the_ard_layout() {
    let source = "com.apple.keylayout.ABC";
    let mut bytes = Vec::new();
    write_apple_set_input_source(&mut bytes, source).unwrap();
    let mut expected = vec![0x1a, 0, 0, 27, 0, 1, 0, 23];
    expected.extend_from_slice(source.as_bytes());
    assert_eq!(
        bytes, expected,
        "SetKeyboardInputSource is the type, padding, the size, version 1, the name length and the name"
    );
    assert_eq!(
        ClientMessage::read(&mut &bytes[..]).unwrap(),
        ClientMessage::AppleSetInputSource { source: source.to_owned() },
        "the server side reads the input source back"
    );
}
