use flate2::{Decompress, FlushDecompress};

use crate::{ClientMessage, write_apple_clipboard};

#[test]
fn test_apple_clipboard_text_travels_as_a_compressed_pasteboard() {
    let text = "Привет, Mac";
    let mut bytes = Vec::new();
    write_apple_clipboard(&mut bytes, 7, text).unwrap();

    assert_eq!(bytes[..8], [0x1f, 0, 0, 0, 0, 0, 0, 7], "the type, format 0, padding and the session id lead");
    let uncompressed = u32::from_be_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let compressed = u32::from_be_bytes(bytes[12..16].try_into().unwrap()) as usize;
    assert_eq!(bytes.len(), 16 + compressed, "the compressed size counts every byte that follows");
    let mut pasteboard = Vec::with_capacity(uncompressed);
    Decompress::new(true)
        .decompress_vec(&bytes[16..], &mut pasteboard, FlushDecompress::Sync)
        .unwrap();
    let mut expected = vec![0, 0, 0, 1, 0, 0, 0, 22];
    expected.extend_from_slice(b"public.utf8-plain-text");
    expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
    expected.extend_from_slice(&(text.len() as u32).to_be_bytes());
    expected.extend_from_slice(text.as_bytes());
    assert_eq!(pasteboard, expected, "one UTF-8 text type with no flags and no properties");
    assert_eq!(uncompressed, expected.len(), "the uncompressed size is the pasteboard length");

    assert_eq!(
        ClientMessage::read(&mut &bytes[..]).unwrap(),
        ClientMessage::AppleClipboard { session_id: 7, text: Some(text.to_owned()) },
        "the server side reads the Cyrillic text back"
    );
}
