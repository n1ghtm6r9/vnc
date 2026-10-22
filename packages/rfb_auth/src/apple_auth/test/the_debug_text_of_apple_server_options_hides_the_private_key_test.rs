use crate::AppleServerOptions;

#[test]
fn test_the_debug_text_of_apple_server_options_hides_the_private_key() {
    let options = AppleServerOptions {
        generator: vec![2],
        prime: vec![23],
        private_key: vec![171, 205],
    };
    let text = format!("{options:?}");
    assert!(
        !text.contains("171") && !text.contains("205"),
        "the debug text keeps the private key out of logs, got {text}"
    );
    assert!(
        text.contains("23"),
        "the debug text still shows the public prime, got {text}"
    );
}
