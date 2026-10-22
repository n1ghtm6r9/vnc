use crate::generate_secrets;

#[test]
fn test_secrets_are_fresh_and_well_formed() {
    let first = generate_secrets().expect("the system gives random bytes");
    let second = generate_secrets().expect("the system gives random bytes");

    assert_eq!(
        first.session_id[6] >> 4,
        4,
        "the session id is a version 4 UUID"
    );
    assert_eq!(
        first.session_id[8] >> 6,
        2,
        "the session id has the RFC 4122 variant"
    );
    assert!(
        first.audio_ssrc != 0 && first.video_ssrc != 0,
        "no source is zero"
    );
    for id in &first.call_ids {
        let groups: Vec<usize> = id.split('-').map(str::len).collect();
        assert_eq!(
            groups,
            [8, 4, 4, 4, 12],
            "the call id {id} is a UUID in five groups"
        );
        assert!(
            id.chars()
                .all(|c| c == '-' || c.is_ascii_digit() || c.is_ascii_uppercase()),
            "the call id {id} is upper-case hex"
        );
        assert_eq!(&id[14..15], "4", "the call id {id} is a version 4 UUID");
    }
    assert_ne!(
        first.call_ids[0], first.call_ids[1],
        "audio and video have their own call ids"
    );
    assert_ne!(
        first.audio_keys, second.audio_keys,
        "every session has new keys"
    );
    assert_ne!(
        first.audio_keys, first.video_keys,
        "audio and video have their own keys"
    );
}
