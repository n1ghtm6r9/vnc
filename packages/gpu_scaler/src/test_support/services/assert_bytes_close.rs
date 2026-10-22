pub fn assert_bytes_close(actual: &[u8], expected: &[u8], tolerance: u8, what: &str) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{what}: the two images differ in size"
    );
    for (index, (&got, &want)) in actual.iter().zip(expected).enumerate() {
        assert!(
            got.abs_diff(want) <= tolerance,
            "{what}: pixel {} channel {} is {got} where {want} was expected, more than {tolerance} apart",
            index / 4,
            index % 4
        );
    }
}
