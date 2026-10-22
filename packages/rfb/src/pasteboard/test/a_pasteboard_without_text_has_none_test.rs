use crate::apple_pasteboard_text;

#[test]
fn test_a_pasteboard_without_text_has_none() {
    let mut pasteboard = 1u32.to_be_bytes().to_vec();
    pasteboard.extend_from_slice(&10u32.to_be_bytes());
    pasteboard.extend_from_slice(b"public.png");
    pasteboard.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0x89, 0x50]);
    assert_eq!(apple_pasteboard_text(&pasteboard), None, "a picture alone gives no text");
}
