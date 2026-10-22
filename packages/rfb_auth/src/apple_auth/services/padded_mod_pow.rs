use num_bigint::BigUint;

pub(crate) fn padded_mod_pow(base: &[u8], exponent: &[u8], modulus: &[u8]) -> Vec<u8> {
    let modulus_value = BigUint::from_bytes_be(modulus);
    if modulus_value == BigUint::ZERO {
        return vec![0u8; modulus.len()];
    }
    let power =
        BigUint::from_bytes_be(base).modpow(&BigUint::from_bytes_be(exponent), &modulus_value);
    let digits = power.to_bytes_be();
    let mut padded = vec![0u8; modulus.len().saturating_sub(digits.len())];
    padded.extend_from_slice(&digits);
    padded
}
