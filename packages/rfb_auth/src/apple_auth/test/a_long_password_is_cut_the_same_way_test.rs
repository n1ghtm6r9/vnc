use crate::apple_credentials_block;

#[test]
fn test_a_long_password_is_cut_the_same_way() {
    let password = "ж".repeat(35);
    let filler = [0x55; 128];
    let block = apple_credentials_block("bogdan", &password, &filler);
    assert_eq!(
        &block[..7],
        b"bogdan\0",
        "the user name is untouched by a long password"
    );
    assert_eq!(
        block[7..64],
        filler[7..64],
        "the rest of the user name field keeps the filler"
    );
    assert_eq!(
        &block[64..126],
        "ж".repeat(31).as_bytes(),
        "31 whole letters of the password are kept"
    );
    assert_eq!(
        block[126], 0,
        "a NUL ends the password after the last whole letter"
    );
    assert_eq!(block[127], 0x55, "the last byte keeps the filler");
}
