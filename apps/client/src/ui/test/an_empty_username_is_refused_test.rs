use super::valid_form::valid_form;
use crate::EMPTY_USERNAME_ERROR;

#[test]
fn test_an_empty_username_is_refused() {
    let mut form = valid_form();
    form.username = String::new();
    assert_eq!(form.validate().unwrap_err(), EMPTY_USERNAME_ERROR, "the Mac login needs the macOS user name");
}
