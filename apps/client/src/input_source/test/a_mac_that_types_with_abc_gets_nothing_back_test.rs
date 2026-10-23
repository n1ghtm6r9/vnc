use crate::{ABC_INPUT_SOURCE, MacInputSource};

#[test]
fn test_a_mac_that_types_with_abc_gets_nothing_back() {
    let mut mac = MacInputSource::default();
    mac.report(ABC_INPUT_SOURCE);
    assert_eq!(mac.give_back(), None, "a Mac that typed with ABC from the start keeps it");
}
