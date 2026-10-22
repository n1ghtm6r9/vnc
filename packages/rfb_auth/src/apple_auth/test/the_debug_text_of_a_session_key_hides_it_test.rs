use crate::AppleSessionKey;

#[test]
fn test_the_debug_text_of_a_session_key_hides_it() {
    let text = format!("{:?}", AppleSessionKey([171; 16]));
    assert!(
        !text.contains("171"),
        "the debug text keeps the session key out of logs, got {text}"
    );
}
