use crate::{dh_public_key, dh_shared_secret};

#[test]
fn test_dh_keys_are_left_padded_to_the_prime_length() {
    let small_prime = [0, 0, 0, 23];
    assert_eq!(
        dh_public_key(&[5], &small_prime, &[4]),
        vec![0, 0, 0, 4],
        "5^4 mod 23 is 4, written in the four bytes of the prime"
    );
    assert_eq!(
        dh_public_key(&[0, 5], &small_prime, &[3]),
        vec![0, 0, 0, 10],
        "5^3 mod 23 is 10, whatever the width of the generator"
    );
    assert_eq!(
        dh_shared_secret(&[0, 0, 0, 10], &[4], &small_prime),
        vec![0, 0, 0, 18],
        "10^4 mod 23 is 18, left-padded like the public keys"
    );
    assert_eq!(
        dh_shared_secret(&[4], &[3], &small_prime),
        vec![0, 0, 0, 18],
        "the other side computes the same secret"
    );
    assert_eq!(
        dh_public_key(&[0], &small_prime, &[1]),
        vec![0, 0, 0, 0],
        "a zero key still fills the whole width"
    );

    let mersenne_127 = [
        0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff,
    ];
    let mut expected = vec![0u8; 16];
    expected[15] = 8;
    assert_eq!(
        dh_public_key(&[2], &mersenne_127, &[3]),
        expected,
        "2^3 mod 2^127-1 is 8 with fifteen zero bytes in front of it"
    );
}
