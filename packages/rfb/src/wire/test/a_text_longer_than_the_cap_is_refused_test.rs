use std::io;

use crate::{MAX_TEXT_LENGTH, read_text};

#[test]
fn test_a_text_longer_than_the_cap_is_refused() {
    let bytes = (MAX_TEXT_LENGTH + 1).to_be_bytes();
    let error = read_text(&mut &bytes[..]).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidData, "a length above the cap is refused before anything is allocated");
}
