use crate::DecodeError;
use crate::test_support::fresh_tight_stream_after_history;

#[test]
fn test_a_tight_reset_bit_restarts_its_stream() {
    fresh_tight_stream_after_history(0x01)
        .expect("a fresh stream after a reset of stream 0 decodes");
    let without_reset = fresh_tight_stream_after_history(0x00);
    assert!(
        matches!(without_reset, Err(DecodeError::Corrupt(_))),
        "a fresh stream header in the middle of stream 0 is corrupt, got {without_reset:?}"
    );
    let other_stream_reset = fresh_tight_stream_after_history(0x0E);
    assert!(
        matches!(other_stream_reset, Err(DecodeError::Corrupt(_))),
        "resetting the other streams leaves stream 0 alone, got {other_stream_reset:?}"
    );
}
