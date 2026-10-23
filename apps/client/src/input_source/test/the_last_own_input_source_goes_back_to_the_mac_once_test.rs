use crate::{ABC_INPUT_SOURCE, MacInputSource};

#[test]
fn test_the_last_own_input_source_goes_back_to_the_mac_once() {
    let mut mac = MacInputSource::default();
    for source in ["com.apple.keylayout.Russian", ABC_INPUT_SOURCE, "com.apple.keylayout.Ukrainian", ABC_INPUT_SOURCE] {
        mac.report(source);
    }
    assert_eq!(
        mac.give_back().as_deref(),
        Some("com.apple.keylayout.Ukrainian"),
        "the Mac gets back the last input source it chose itself"
    );
    assert_eq!(mac.give_back(), None, "the input source goes back only once");
}
