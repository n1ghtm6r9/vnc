use rfb_auth::{AppleServerOptions, RFC2409_GROUP2_PRIME, random_bytes};

pub fn apple_server_options() -> AppleServerOptions {
    AppleServerOptions {
        generator: vec![2],
        prime: RFC2409_GROUP2_PRIME.to_vec(),
        private_key: random_bytes(128),
    }
}
