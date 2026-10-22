use crate::random_bytes;

#[test]
fn test_random_bytes_have_the_requested_length_and_differ_between_calls() {
    let first = random_bytes(32);
    let second = random_bytes(32);
    assert_eq!(first.len(), 32, "as many bytes as asked for");
    assert_ne!(first, second, "two calls never return the same 32 bytes");
    assert!(
        random_bytes(0).is_empty(),
        "asking for nothing returns nothing"
    );
}
