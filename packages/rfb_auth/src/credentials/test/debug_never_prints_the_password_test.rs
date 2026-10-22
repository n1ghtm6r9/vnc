use crate::Credentials;

#[test]
fn test_debug_never_prints_the_password() {
    let credentials = Credentials::new("bogdan", "hunter2-секрет");
    let compact = format!("{credentials:?}");
    let pretty = format!("{credentials:#?}");
    assert!(
        compact.contains("bogdan"),
        "the user name is printed, so a log line says whose login it was"
    );
    assert!(
        compact.contains("\"***\""),
        "the password shows as three stars"
    );
    assert!(
        !compact.contains("hunter2") && !compact.contains("секрет"),
        "the password never reaches a log line"
    );
    assert!(
        !pretty.contains("hunter2") && !pretty.contains("секрет"),
        "the pretty form hides the password too"
    );
}
