use super::valid_form::valid_form;
use crate::EMPTY_HOST_ERROR;

#[test]
fn test_an_empty_host_is_refused() {
    let mut form = valid_form();
    form.host = "   ".to_string();
    assert_eq!(form.validate().unwrap_err(), EMPTY_HOST_ERROR, "a blank address cannot be connected to");
}
