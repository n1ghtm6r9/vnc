pub fn should_reconnect_without_high_performance(online: bool, high_performance: bool, picture_shown: bool) -> bool {
    online && high_performance && !picture_shown
}
