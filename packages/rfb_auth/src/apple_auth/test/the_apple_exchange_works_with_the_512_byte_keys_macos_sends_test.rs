use super::run_apple_exchange::run_apple_exchange;
use crate::{AppleServerOptions, Credentials, random_bytes};

#[test]
fn test_the_apple_exchange_works_with_the_512_byte_keys_macos_sends() {
    let mut prime: Vec<u8> = (0..512usize)
        .map(|index| index.wrapping_mul(167).wrapping_add(31) as u8)
        .collect();
    prime[0] |= 0x80;
    prime[511] |= 1;
    let options = AppleServerOptions {
        generator: vec![0, 5],
        prime,
        private_key: random_bytes(512),
    };
    let credentials = Credentials::new("bogdan", "пароль");

    let (client_result, server_result) = run_apple_exchange(options, &credentials);

    client_result.unwrap();
    assert_eq!(
        server_result.unwrap().credentials,
        credentials,
        "512-byte keys with the generator 5 that macOS sends carry the same credentials through"
    );
}
