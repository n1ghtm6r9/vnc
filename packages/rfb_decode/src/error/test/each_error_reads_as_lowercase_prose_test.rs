use rfb::Rect;

use crate::DecodeError;

#[test]
fn test_each_error_reads_as_lowercase_prose() {
    let messages = [
        DecodeError::UnsupportedEncoding(99).to_string(),
        DecodeError::RectOutOfBounds(Rect::new(1, 2, 3, 4)).to_string(),
        DecodeError::Corrupt("a tile ended early".to_string()).to_string(),
    ];
    for message in &messages {
        assert!(
            message.starts_with("the server"),
            "the message {message:?} starts with its subject in lowercase"
        );
        assert!(
            !message.ends_with('.'),
            "the message {message:?} has no trailing full stop"
        );
    }
    assert!(
        messages[0].contains("99"),
        "the unsupported encoding number is named"
    );
    assert!(
        messages[1].contains("3x4") && messages[1].contains("(1, 2)"),
        "the rectangle geometry is named"
    );
    assert!(
        messages[2].contains("a tile ended early"),
        "the corruption reason is kept"
    );
}
