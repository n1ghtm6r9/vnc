use std::{error::Error, io};

use crate::DecodeError;

#[test]
fn test_an_io_error_converts_and_keeps_its_source() {
    let error: DecodeError = io::Error::new(io::ErrorKind::UnexpectedEof, "gone").into();
    assert!(
        matches!(error, DecodeError::Io(_)),
        "an io error converts into the Io variant"
    );
    assert!(
        error.source().is_some(),
        "the io error stays reachable as the source"
    );
    assert!(
        DecodeError::Corrupt(String::new()).source().is_none(),
        "the other variants have no source"
    );
}
