use crate::{RateControl, rate_control_packet, test_support::hex_bytes};

#[test]
fn test_a_rate_control_report_matches_apples_layout() {
    let report = RateControl {
        ssrc: 0x73fd_5806,
        pictures: 0x0011,
        since_last_packet_ms: 1,
        packets: 0x82,
        bandwidth_kbit: 0xea60,
    };

    assert_eq!(
        rate_control_packet(&report).to_vec(),
        hex_bytes("80cc0007 73fd5806 5243544c 85000004 0011 00000000 0001 ffff 00000082 ea60"),
        "the report is an APP packet named RCTL"
    );
}
