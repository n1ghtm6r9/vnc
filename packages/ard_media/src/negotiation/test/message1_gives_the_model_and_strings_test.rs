use super::fixtures::message1;
use crate::parse_message1;

#[test]
fn test_message1_gives_the_model_and_strings() {
    let message = parse_message1(&message1(&["Macmini9,1", "unknown", "unknown"]))
        .expect("a well-formed Message 1 reads");

    assert_eq!(
        message.model, "Macmini9,1",
        "the first string names the Mac model"
    );
    assert_eq!(
        message.strings,
        ["Macmini9,1", "unknown", "unknown"],
        "all three strings are read without their terminating zeros"
    );
}
