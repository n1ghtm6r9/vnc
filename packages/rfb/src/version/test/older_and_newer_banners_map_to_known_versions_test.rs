use crate::ProtocolVersion;

#[test]
fn test_older_and_newer_banners_map_to_known_versions() {
    let cases: [(&[u8; 12], ProtocolVersion); 5] = [
        (b"RFB 003.003\n", ProtocolVersion::V3_3),
        (b"RFB 003.005\n", ProtocolVersion::V3_3),
        (b"RFB 003.007\n", ProtocolVersion::V3_7),
        (b"RFB 003.008\n", ProtocolVersion::V3_8),
        (b"RFB 005.000\n", ProtocolVersion::V3_8),
    ];
    for (banner, expected) in cases {
        let version = ProtocolVersion::read_server(&mut &banner[..]).unwrap();
        assert_eq!(version, expected, "{} maps to {expected:?}", String::from_utf8_lossy(banner).trim_end());
    }
}
