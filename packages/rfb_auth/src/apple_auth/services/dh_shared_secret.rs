use super::padded_mod_pow;

pub fn dh_shared_secret(peer_public: &[u8], private_key: &[u8], prime: &[u8]) -> Vec<u8> {
    padded_mod_pow(peer_public, private_key, prime)
}
