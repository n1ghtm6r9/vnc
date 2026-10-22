use crate::apple_key_block;

#[test]
fn test_an_apple_key_block_has_the_ard_layout() {
    let block = apple_key_block(0xff0d, true, 0x0102_0304);
    assert_eq!(
        block,
        [0xff, 1, 0, 0, 0xff, 0x0d, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0],
        "a marker byte, the down flag, the keysym and the timestamp big-endian, then six zero bytes"
    );
}
