use super::run_apple_exchange::run_apple_exchange;
use crate::{AppleServerOptions, Credentials, RFC2409_GROUP2_PRIME, random_bytes};

#[test]
fn test_a_cyrillic_password_reaches_the_server_as_utf8() {
    let options = AppleServerOptions {
        generator: vec![0, 2],
        prime: RFC2409_GROUP2_PRIME.to_vec(),
        private_key: random_bytes(128),
    };
    let credentials = Credentials::new("богдан", "пароль");

    let (client_result, server_result) = run_apple_exchange(options, &credentials);

    client_result.unwrap();
    let received = server_result.unwrap().credentials;
    assert_eq!(
        received.password, "пароль",
        "the Cyrillic password arrives as the same text"
    );
    assert_eq!(
        received.username, "богдан",
        "a Cyrillic user name arrives as the same text"
    );
}
