use std::time::Duration;

use crate::ntp_time;

#[test]
fn test_unix_time_converts_to_the_ntp_form_of_the_offers() {
    assert_eq!(
        ntp_time(Duration::ZERO),
        2_208_988_800 << 32,
        "the Unix epoch is 2208988800 seconds into the NTP era"
    );
    assert_eq!(
        ntp_time(Duration::from_millis(1500)),
        (2_208_988_801 << 32) | 0x8000_0000,
        "half a second is half of the 32-bit fraction"
    );
}
