use std::time::{Duration, Instant};

use crate::{
    Datagram, RATE_CONTROL_INTERVAL,
    test_support::{Control, streaming_session},
};

#[test]
fn test_rate_control_goes_out_every_fifty_milliseconds() {
    let start = Instant::now();
    let (mut session, server) = streaming_session(start);
    let rate_controls = |datagrams: &[Datagram]| -> Vec<Control> {
        server
            .open_all(datagrams)
            .into_iter()
            .filter(|control| matches!(control, Control::Rate { .. }))
            .collect()
    };

    session.tick(start + RATE_CONTROL_INTERVAL - Duration::from_millis(1));
    assert!(
        rate_controls(&session.effects.take().datagrams).is_empty(),
        "nothing goes out before fifty milliseconds have passed"
    );

    session.tick(start + RATE_CONTROL_INTERVAL);
    assert_eq!(
        rate_controls(&session.effects.take().datagrams),
        [Control::Rate {
            pictures: 1,
            packets: 1
        }],
        "the rate control counts the one picture and packet so far"
    );
}
