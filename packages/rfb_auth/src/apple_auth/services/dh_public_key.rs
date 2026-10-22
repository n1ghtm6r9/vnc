use super::padded_mod_pow;

pub fn dh_public_key(generator: &[u8], prime: &[u8], private_key: &[u8]) -> Vec<u8> {
    padded_mod_pow(generator, private_key, prime)
}
