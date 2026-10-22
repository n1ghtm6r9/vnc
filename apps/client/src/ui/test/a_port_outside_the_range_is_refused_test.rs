use super::valid_form::valid_form;
use crate::INVALID_PORT_ERROR;

#[test]
fn test_a_port_outside_the_range_is_refused() {
    for port in ["0", "65536", "abc", ""] {
        let mut form = valid_form();
        form.port = port.to_string();
        assert_eq!(form.validate().unwrap_err(), INVALID_PORT_ERROR, "port {port:?} is not a TCP port");
    }
}
