use crate::{MasterKey, derive_session_keys, test_support::hex_bytes};

#[test]
fn test_session_keys_match_rfc_6188() {
    let master = MasterKey {
        key: hex_bytes("f0f04914b513f2763a1b1fa130f10e2998f6f6e43e4309d1e622a0e332b9f1b6")
            .try_into()
            .expect("32 bytes"),
        salt: hex_bytes("3b04803de51ee7c96423ab5b78d2")
            .try_into()
            .expect("14 bytes"),
    };

    let keys = derive_session_keys(&master);

    assert_eq!(
        keys.rtp.cipher_key.to_vec(),
        hex_bytes("5ba1064e30ec51613cad926c5a28ef731ec7fb397f70a960653caf06554cd8c4"),
        "the RTP cipher key is the one in RFC 6188 section 7.1"
    );
    assert_eq!(
        keys.rtp.auth_key.to_vec(),
        hex_bytes("fd9c32d39ed5fbb5a9dc96b30818454d1313dc05"),
        "the RTP authentication key is the one in RFC 6188 section 7.1"
    );
    assert_eq!(
        keys.rtp.salt.to_vec(),
        hex_bytes("fa31791685ca444a9e07c6c64e93"),
        "the RTP session salt is the one in RFC 6188 section 7.1"
    );
}
