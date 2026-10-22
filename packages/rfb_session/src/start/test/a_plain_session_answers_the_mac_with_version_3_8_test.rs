use crate::test_support::{FakeServer, serve_version, start_with_events, test_options};

#[test]
fn test_a_plain_session_answers_the_mac_with_version_3_8() {
    let server = FakeServer::start(serve_version);
    let (_session, _events) = start_with_events(test_options(server.port()));

    assert_eq!(
        &server.outcome(),
        b"RFB 003.008\n",
        "without Apple Remote Desktop mode the client speaks plain RFB 3.8 to the Mac"
    );
}
