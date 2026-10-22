use crate::apple_credentials_block;

#[test]
fn test_the_63_byte_limit_is_exact() {
    let filler = [0x77; 128];
    let exact = apple_credentials_block(&"a".repeat(63), "", &filler);
    assert_eq!(
        &exact[..63],
        "a".repeat(63).as_bytes(),
        "a name of 63 bytes is kept whole"
    );
    assert_eq!(exact[63], 0, "the NUL takes the 64th byte");
    let over = apple_credentials_block(&"a".repeat(64), "", &filler);
    assert_eq!(
        &over[..63],
        "a".repeat(63).as_bytes(),
        "a name of 64 bytes loses its last byte"
    );
    assert_eq!(over[63], 0, "the NUL still takes the 64th byte");
    assert_eq!(over[64], 0, "an empty password is just a NUL");
    assert_eq!(
        over[65..],
        filler[65..],
        "the rest of the password field keeps the filler"
    );
}
