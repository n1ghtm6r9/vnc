use crate::apple_pasteboard_text;

#[test]
fn test_the_text_type_is_found_after_other_types_and_properties() {
    let field = |bytes: &mut Vec<u8>, data: &[u8]| {
        bytes.extend_from_slice(&(data.len() as u32).to_be_bytes());
        bytes.extend_from_slice(data);
    };
    let mut pasteboard = 2u32.to_be_bytes().to_vec();
    field(&mut pasteboard, b"public.rtf");
    pasteboard.extend_from_slice(&0u32.to_be_bytes());
    pasteboard.extend_from_slice(&1u32.to_be_bytes());
    field(&mut pasteboard, b"key");
    field(&mut pasteboard, b"value");
    field(&mut pasteboard, b"{\\rtf1 bold}");
    field(&mut pasteboard, b"public.utf8-plain-text");
    pasteboard.extend_from_slice(&7u32.to_be_bytes());
    pasteboard.extend_from_slice(&0u32.to_be_bytes());
    field(&mut pasteboard, "текст".as_bytes());
    assert_eq!(
        apple_pasteboard_text(&pasteboard),
        Some("текст".to_owned()),
        "rich text and its properties are skipped on the way to the plain text"
    );
}
