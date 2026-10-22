use crate::apple_credentials_block;

#[test]
fn test_a_long_username_is_cut_at_a_character_boundary() {
    let username = "ж".repeat(35);
    assert_eq!(
        username.len(),
        70,
        "35 Cyrillic letters are 70 bytes of UTF-8"
    );
    let filler = [0xaa; 128];
    let block = apple_credentials_block(&username, "secret", &filler);
    assert_eq!(
        &block[..62],
        "ж".repeat(31).as_bytes(),
        "31 whole letters fit into the 63 allowed bytes"
    );
    assert_eq!(
        block[62], 0,
        "a NUL ends the user name right after the last whole letter"
    );
    assert_eq!(block[63], 0xaa, "the byte after the NUL keeps the filler");
    assert_eq!(
        &block[64..71],
        b"secret\0",
        "the password starts at offset 64 and ends with a NUL"
    );
    assert_eq!(
        block[71..],
        filler[71..],
        "the rest of the password field keeps the filler"
    );
}
