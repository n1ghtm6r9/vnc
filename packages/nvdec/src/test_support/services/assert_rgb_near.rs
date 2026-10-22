use super::rgb_near;

pub fn assert_rgb_near(actual: [u8; 3], expected: [u8; 3], what: &str) {
    assert!(
        rgb_near(actual, expected),
        "{what} comes out as {actual:?} instead of about {expected:?}"
    );
}
