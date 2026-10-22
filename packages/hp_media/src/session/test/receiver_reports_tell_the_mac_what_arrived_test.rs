use std::time::{Duration, Instant};

use crate::{
    MediaChannel, REPORT_INTERVAL,
    test_support::{
        Control, FIRST_DON, FIRST_SEQ, NAL_TRAIL, TEST_VIDEO_SSRC, deliver, fake_unit,
        streaming_session,
    },
};

#[test]
fn test_receiver_reports_tell_the_mac_what_arrived() {
    let start = Instant::now();
    let (mut session, mut server) = streaming_session(start);
    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON + 1, 0, &[&fake_unit(NAL_TRAIL, 2)]),
        start,
    );
    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON + 2, 1, &[&fake_unit(NAL_TRAIL, 2)]),
        start,
    );
    let mut sender_report = server.sender_report(MediaChannel::Video, 0x1234_5678);
    session.handle_datagram(
        MediaChannel::Video,
        &mut sender_report,
        start + Duration::from_millis(100),
    );
    session.effects.take();

    session.tick(start + REPORT_INTERVAL);

    let reports: Vec<Control> = server
        .open_all(&session.effects.take().datagrams)
        .into_iter()
        .filter(|control| matches!(control, Control::Report { .. }))
        .collect();
    assert_eq!(
        reports,
        [Control::Report {
            channel: MediaChannel::Video,
            source_ssrc: TEST_VIDEO_SSRC,
            highest_seq: u32::from(FIRST_SEQ + 1),
            last_sender_report: 0x1234_5678,
            delay: 58982,
        }],
        "the video report names the newest packet of the first stripe and the sender report 0.9 s ago; no audio came, so no audio report"
    );
}
