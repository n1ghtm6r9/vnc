use crate::{LinkState, overlay_hears_keys};

#[test]
fn test_the_connect_form_hears_the_keys() {
    assert!(overlay_hears_keys(LinkState::Offline), "the address and the password are typed into the connect form");
}
