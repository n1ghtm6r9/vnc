use rfb::{ENCODING_APPLE_USER_INFO, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_the_apple_user_info_is_skipped() {
    let mut payload = vec![0, 4];
    payload.extend_from_slice(b"user");
    payload.extend_from_slice(&3u32.to_be_bytes());
    payload.extend_from_slice(&6u32.to_be_bytes());
    payload.extend_from_slice(&[1, 2, 3]);
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(ENCODING_APPLE_USER_INFO, Rect::new(0, 0, 32, 32), &payload)
        .expect("a user info decodes");
    assert_eq!(
        outcome,
        RectOutcome::Ignored,
        "the name and picture of the Mac user are read whole and not used"
    );
}
