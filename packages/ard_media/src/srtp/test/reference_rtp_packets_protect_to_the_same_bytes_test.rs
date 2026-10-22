use super::vectors::{RTP_VECTORS, bytes, reference_master};
use crate::SrtpContext;

#[test]
fn test_reference_rtp_packets_protect_to_the_same_bytes() {
    let mut context = SrtpContext::new(&reference_master());

    for (name, plain, protected) in RTP_VECTORS {
        assert_eq!(
            context.protect_rtp(&bytes(plain)),
            Some(bytes(protected)),
            "{name}: protecting gives the reference bytes, the rollover counter included"
        );
    }
}
