use crate::Settings;

#[test]
fn test_high_performance_is_off_by_default() {
    assert!(
        !Settings::default().high_performance,
        "a fresh install and an older settings file keep the Mac screen on until High Performance is chosen"
    );
}
