use num_bigint::BigUint;

use super::run_apple_exchange::run_apple_exchange;
use crate::{AppleServerOptions, Credentials, random_bytes};

#[test]
fn test_the_apple_exchange_works_with_a_2048_bit_group() {
    let rfc3526_group14_prime = concat!(
        "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1",
        "29024E088A67CC74020BBEA63B139B22514A08798E3404DD",
        "EF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245",
        "E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7ED",
        "EE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3D",
        "C2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F",
        "83655D23DCA3AD961C62F356208552BB9ED529077096966D",
        "670C354E4ABC9804F1746C08CA18217C32905E462E36CE3B",
        "E39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9",
        "DE2BCBF6955817183995497CEA956AE515D2261898FA0510",
        "15728E5A8AACAA68FFFFFFFFFFFFFFFF",
    );
    let prime = BigUint::parse_bytes(rfc3526_group14_prime.as_bytes(), 16)
        .unwrap()
        .to_bytes_be();
    assert_eq!(prime.len(), 256, "a 2048-bit group has 256-byte keys");
    let options = AppleServerOptions {
        generator: vec![0, 2],
        prime,
        private_key: random_bytes(256),
    };
    let credentials = Credentials::new("bogdan", "пароль");

    let (client_result, server_result) = run_apple_exchange(options, &credentials);

    client_result.unwrap();
    assert_eq!(
        server_result.unwrap().credentials,
        credentials,
        "the 256-byte keys carry the same credentials through"
    );
}
