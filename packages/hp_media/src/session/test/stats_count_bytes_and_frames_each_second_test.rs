use std::time::Instant;

use crate::{
    MediaChannel, MediaEvent, MediaStats, STATS_INTERVAL,
    test_support::{
        FIRST_DON, FakeDecoder, NAL_IDR, NAL_TRAIL, TestServer, answered_session, deliver,
        fake_unit,
    },
};

#[test]
fn test_stats_count_bytes_and_frames_each_second() {
    let start = Instant::now();
    let mut session = answered_session(FakeDecoder::default(), start);
    let mut server = TestServer::new();
    let mut datagrams = server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 1)]);
    for (step, stripe) in [(1, 1), (2, 0), (3, 1)] {
        datagrams.extend(server.picture(FIRST_DON + step, stripe, &[&fake_unit(NAL_TRAIL, 1)]));
    }
    let bytes: usize = datagrams.iter().map(Vec::len).sum();
    deliver(&mut session, MediaChannel::Video, datagrams, start);
    session.effects.take();

    session.tick(start + STATS_INTERVAL);

    let stats: Vec<MediaStats> = session
        .effects
        .take()
        .events
        .into_iter()
        .filter_map(|event| match event {
            MediaEvent::Stats(stats) => Some(stats),
            _ => None,
        })
        .collect();
    assert_eq!(
        stats,
        [MediaStats {
            bytes_per_second: bytes as u64,
            frames_per_second: 2.0,
        }],
        "four stripes of a two-stripe picture make two frames in the second"
    );
}
