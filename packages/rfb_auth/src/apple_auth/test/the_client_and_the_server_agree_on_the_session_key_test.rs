use super::run_apple_exchange::run_apple_exchange;
use crate::{AppleServerOptions, Credentials, RFC2409_GROUP2_PRIME, random_bytes};

#[test]
fn test_the_client_and_the_server_agree_on_the_session_key() {
    let options = AppleServerOptions {
        generator: vec![0, 2],
        prime: RFC2409_GROUP2_PRIME.to_vec(),
        private_key: random_bytes(128),
    };

    let (client_result, server_result) =
        run_apple_exchange(options, &Credentials::new("user", "secret"));

    let client_key = client_result.unwrap();
    let login = server_result.unwrap();
    assert_eq!(
        client_key, login.key,
        "both sides derive the key for encrypted events from the shared secret"
    );
    let block = [7u8; 16];
    assert_eq!(
        login.key.decrypt(&client_key.encrypt(&block)),
        block,
        "what the client encrypts the server reads back"
    );
}
