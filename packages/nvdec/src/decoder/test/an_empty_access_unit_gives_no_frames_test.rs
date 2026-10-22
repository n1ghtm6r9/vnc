use crate::test_support::test_decoder;

#[test]
fn test_an_empty_access_unit_gives_no_frames() {
    let Some(mut decoder) = test_decoder() else {
        return;
    };

    let frames = decoder
        .decode(&[])
        .expect("an empty access unit is not an error");

    assert!(frames.is_empty(), "an empty access unit gives no frames");
}
