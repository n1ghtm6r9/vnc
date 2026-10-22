use rfb_session::SessionError;

use crate::should_reconnect;

#[test]
fn test_a_session_the_mac_user_ended_is_not_reconnected() {
    assert!(
        !should_reconnect(Some(&SessionError::ClosedOnTheMac), true),
        "a session the user of the Mac ended on purpose is not opened again, so the client does not take the Mac back"
    );
}
