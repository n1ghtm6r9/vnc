use super::vectors::reference_master;
use crate::{derive_session_keys, test_support::hex_bytes};

#[test]
fn test_session_keys_match_the_reference_for_all_six_labels() {
    let keys = derive_session_keys(&reference_master());

    let cases = [
        (
            keys.rtp.cipher_key.to_vec(),
            "8ff338062f0014ad7018ceda7ad30d784b582faa338d95c29f02aab161a7f31f",
            "RTP cipher key",
        ),
        (
            keys.rtp.auth_key.to_vec(),
            "b916d2a76f03cb2e48ab6252a31f228a41f248b7",
            "RTP authentication key",
        ),
        (
            keys.rtp.salt.to_vec(),
            "d77cbb7a53e2e86a097453231e04",
            "RTP salt",
        ),
        (
            keys.rtcp.cipher_key.to_vec(),
            "618488fb3a5f72a110bd921dd2c5f34ceca03ce395457d6d4451d122d4dc3b61",
            "RTCP cipher key",
        ),
        (
            keys.rtcp.auth_key.to_vec(),
            "a7111d61dc1271672d1c73d5f13c14d85a999e86",
            "RTCP authentication key",
        ),
        (
            keys.rtcp.salt.to_vec(),
            "2999a34fecf08950cee6db7e19e1",
            "RTCP salt",
        ),
    ];
    for (derived, expected, meaning) in cases {
        assert_eq!(
            derived,
            hex_bytes(expected),
            "the {meaning} matches the reference decoder"
        );
    }
}
