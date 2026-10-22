use crate::vnc_auth_response;

#[test]
fn test_a_password_is_cut_to_eight_bytes() {
    let challenge: [u8; 16] =
        std::array::from_fn(|index| (index as u8).wrapping_mul(13).wrapping_add(1));
    assert_eq!(
        vnc_auth_response(&challenge, "password"),
        vnc_auth_response(&challenge, "password and more"),
        "bytes after the eighth do not change the response"
    );
    assert_ne!(
        vnc_auth_response(&challenge, "password"),
        vnc_auth_response(&challenge, "passwor"),
        "the eighth byte does count"
    );
    assert_eq!(
        vnc_auth_response(&challenge, "abc"),
        vnc_auth_response(&challenge, "abc\0\0\0\0\0"),
        "a short password is padded with zero bytes"
    );
}
