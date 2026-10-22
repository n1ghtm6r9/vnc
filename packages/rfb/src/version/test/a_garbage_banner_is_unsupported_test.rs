use crate::{ProtocolError, ProtocolVersion};

#[test]
fn test_a_garbage_banner_is_unsupported() {
    let banners: [&[u8; 12]; 3] = [b"HTTP/1.1 400", b"RFB 0x3.008\n", b"RFB \xff\xfe\xfd.\xfc\xfb\xfa\n"];
    for banner in banners {
        let error = ProtocolVersion::read_server(&mut &banner[..]).unwrap_err();
        assert!(
            matches!(error, ProtocolError::UnsupportedVersion(_)),
            "a banner that is not RFB xxx.yyy must be refused without a panic, got {error:?}"
        );
    }
}
