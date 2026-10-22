use super::vectors::{RTCP_VECTORS, bytes, reference_master};
use crate::SrtpContext;

#[test]
fn test_reference_srtcp_packets_decrypt() {
    let context = SrtpContext::new(&reference_master());

    for (name, plain, protected) in RTCP_VECTORS {
        let mut wire = bytes(protected);
        assert_eq!(
            context.unprotect_rtcp(&mut wire).map(<[u8]>::to_vec),
            Some(bytes(plain)),
            "{name}: the control packet verifies and decrypts"
        );
    }
}
