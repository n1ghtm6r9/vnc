use crate::{ReceiverReport, receiver_report_packet, test_support::hex_bytes};

#[test]
fn test_a_receiver_report_matches_apples_layout() {
    let plain = ReceiverReport {
        ssrc: 0x73fd_5806,
        source_ssrc: 0x1267_1595,
        highest_seq: 0x6361,
        ..ReceiverReport::default()
    };
    let timed = ReceiverReport {
        highest_seq: 0x6e9c,
        jitter: 0x667,
        last_sender_report: 0x01bb_8eb5,
        delay_since_sender_report: 0x0bf1,
        ..plain
    };
    let lossy = ReceiverReport {
        fraction_lost: 0x40,
        cumulative_lost: 0x0102_0304,
        ..plain
    };

    assert_eq!(
        receiver_report_packet(&plain).to_vec(),
        hex_bytes(
            "81c90007 73fd5806 12671595 00000000 00006361 00000000 00000000 00000000 81ca0002 73fd5806 01000000"
        ),
        "a report with only a sequence number, followed by an empty CNAME"
    );
    assert_eq!(
        receiver_report_packet(&timed).to_vec(),
        hex_bytes(
            "81c90007 73fd5806 12671595 00000000 00006e9c 00000667 01bb8eb5 00000bf1 81ca0002 73fd5806 01000000"
        ),
        "a report with jitter and the sender report times"
    );
    assert_eq!(
        receiver_report_packet(&lossy)[12..16],
        [0x40, 0xff, 0xff, 0xff],
        "the loss count is capped at 24 bits under the loss fraction"
    );
}
