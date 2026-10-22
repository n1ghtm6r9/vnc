use std::time::{Duration, Instant};

use rfb::{ENCODING_RAW, ENCODING_ZRLE};

use crate::{SessionStats, StatsMeter};

#[test]
fn test_stats_are_reported_at_most_once_per_second() {
    let start = Instant::now();
    let mut meter = StatsMeter::new(start);

    let early = meter.record_update(start + Duration::from_millis(250), 1_000, ENCODING_RAW);
    let still_early = meter.record_update(start + Duration::from_millis(500), 2_000, ENCODING_RAW);
    let report = meter.record_update(start + Duration::from_millis(2_000), 5_000, ENCODING_ZRLE);
    let after_report =
        meter.record_update(start + Duration::from_millis(2_500), 500, ENCODING_ZRLE);

    assert_eq!(early, None, "nothing is reported a quarter of a second in");
    assert_eq!(still_early, None, "nothing is reported half a second in");
    assert_eq!(
        report,
        Some(SessionStats {
            bytes_per_second: 4_000,
            updates_per_second: 1.5,
            last_encoding: ENCODING_ZRLE,
        }),
        "two seconds in, the report averages bytes and updates over the whole window"
    );
    assert_eq!(
        after_report, None,
        "the next report waits for another full second"
    );
}
