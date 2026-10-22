use super::fixtures::message1;
use crate::parse_message1;

#[test]
fn test_message1_with_a_bad_length_is_refused() {
    let full = message1(&["Mac16,10", "unknown", "unknown"]);
    let mut overlong = full.clone();
    overlong[2 + 10] = 0x7f;

    assert!(
        parse_message1(&full[..full.len() - 5]).is_err(),
        "a cut message is refused"
    );
    assert!(
        parse_message1(&[0]).is_err(),
        "a message without its length is refused"
    );
    assert!(
        parse_message1(&[0, 4, 0, 2, 0, 0]).is_err(),
        "a message too short for its header is refused"
    );
    assert!(
        parse_message1(&overlong).is_err(),
        "a string running past the end is refused"
    );
}
