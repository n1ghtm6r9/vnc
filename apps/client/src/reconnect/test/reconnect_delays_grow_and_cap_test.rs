use crate::reconnect_delay;

#[test]
fn test_reconnect_delays_grow_and_cap() {
    let seconds: Vec<u64> = (0..7).map(|attempt| reconnect_delay(attempt).as_secs()).collect();
    assert_eq!(seconds, vec![1, 2, 5, 10, 10, 10, 10], "retries wait 1, 2, 5 and then 10 seconds forever");
    assert_eq!(reconnect_delay(usize::MAX).as_secs(), 10, "a huge attempt count still waits 10 seconds");
}
