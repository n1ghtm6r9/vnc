use super::vectors::{RTCP_VECTORS, RTP_VECTORS, bytes, reference_master};
use crate::{MasterKey, SrtpContext};

#[test]
fn test_a_tampered_or_foreign_packet_is_refused() {
    let (_, _, protected) = RTP_VECTORS[0];
    let (_, _, control) = RTCP_VECTORS[0];
    let mut context = SrtpContext::new(&reference_master());
    let mut stranger = SrtpContext::new(&MasterKey {
        key: [0xff; 32],
        ..reference_master()
    });

    let mut flipped = bytes(protected);
    flipped[20] ^= 1;
    let mut short = bytes(protected)[..30].to_vec();
    let mut foreign = bytes(protected);
    let mut flipped_control = bytes(control);
    flipped_control[9] ^= 0x40;

    assert!(
        context.unprotect_rtp(&mut flipped).is_none(),
        "a changed payload byte fails the tag"
    );
    assert!(
        context.unprotect_rtp(&mut short).is_none(),
        "a cut packet fails the tag"
    );
    assert!(
        context.unprotect_rtp(&mut [0x80; 15]).is_none(),
        "a packet too short for a header and a tag is refused"
    );
    assert!(
        stranger.unprotect_rtp(&mut foreign).is_none(),
        "a packet under other keys is refused"
    );
    assert!(
        context.unprotect_rtcp(&mut flipped_control).is_none(),
        "a changed control packet fails the tag"
    );
    assert!(
        context.unprotect_rtcp(&mut [0x80; 21]).is_none(),
        "a control packet too short for its trailer is refused"
    );
}
