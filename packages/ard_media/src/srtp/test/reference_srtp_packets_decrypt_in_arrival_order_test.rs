use super::vectors::{EXTENSION, RTP_VECTORS, bytes, reference_master};
use crate::SrtpContext;

#[test]
fn test_reference_srtp_packets_decrypt_in_arrival_order() {
    let mut context = SrtpContext::new(&reference_master());

    for (name, plain, protected) in RTP_VECTORS {
        let plain = bytes(plain);
        let mut wire = bytes(protected);
        let packet = context
            .unprotect_rtp(&mut wire)
            .unwrap_or_else(|| panic!("the {name} packet verifies"));
        let has_extension = plain[0] & 0x10 != 0;
        let payload_start = if has_extension { 20 } else { 12 };

        assert_eq!(
            packet.seq,
            u16::from_be_bytes([plain[2], plain[3]]),
            "{name}: sequence number"
        );
        assert_eq!(
            packet.ssrc,
            u32::from_be_bytes([plain[8], plain[9], plain[10], plain[11]]),
            "{name}: source"
        );
        assert_eq!(
            packet.timestamp,
            u32::from_be_bytes([plain[4], plain[5], plain[6], plain[7]]),
            "{name}: timestamp"
        );
        assert_eq!(packet.marker, plain[1] & 0x80 != 0, "{name}: marker bit");
        assert_eq!(packet.payload_type, 100, "{name}: payload type");
        assert_eq!(
            packet.extension,
            if has_extension {
                bytes(EXTENSION)
            } else {
                Vec::new()
            },
            "{name}: the header extension stays readable"
        );
        assert_eq!(
            packet.payload,
            &plain[payload_start..],
            "{name}: the payload is decrypted"
        );
    }
}
