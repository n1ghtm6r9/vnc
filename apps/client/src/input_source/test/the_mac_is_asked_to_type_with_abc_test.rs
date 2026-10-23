use crate::{ABC_INPUT_SOURCE, MacInputSource};

#[test]
fn test_the_mac_is_asked_to_type_with_abc() {
    let mut mac = MacInputSource::default();
    assert_eq!(mac.report("com.apple.keylayout.Russian"), Some(ABC_INPUT_SOURCE), "a Mac on Russian is asked for ABC");
    assert_eq!(mac.report(ABC_INPUT_SOURCE), None, "a Mac that types with ABC is left alone");
}
