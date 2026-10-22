use super::vectors::{RTCP_VECTORS, bytes, reference_master};
use crate::SrtpContext;

#[test]
fn test_reference_rtcp_packets_protect_to_the_same_bytes() {
    let mut context = SrtpContext::new(&reference_master());

    for (name, plain, protected) in RTCP_VECTORS {
        assert_eq!(
            context.protect_rtcp(&bytes(plain)),
            Some(bytes(protected)),
            "{name}: the index counts up from 1 and the bytes match the reference"
        );
    }
    assert_eq!(
        context.protect_rtcp(&[0x80, 0xc9, 0, 1]),
        None,
        "a control packet shorter than its header is not sent"
    );
}
