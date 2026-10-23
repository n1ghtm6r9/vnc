use crate::should_reconnect_without_high_performance;

#[test]
fn test_a_link_lost_while_high_performance_starts_is_reconnected_in_standard_mode() {
    assert!(
        should_reconnect_without_high_performance(true, true, false),
        "a Mac that breaks the link before the first High Performance picture is opened again in standard mode, so the client does not loop"
    );
    assert!(
        !should_reconnect_without_high_performance(true, true, true),
        "a link lost while High Performance shows pictures is opened again in High Performance"
    );
    assert!(
        !should_reconnect_without_high_performance(false, true, false),
        "a reconnect attempt that never got through keeps High Performance for the next one"
    );
}
