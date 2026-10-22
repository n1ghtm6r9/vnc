use crate::{AppleScroll, ClientMessage, write_apple_scroll};

#[test]
fn test_an_apple_scroll_is_a_compact_and_a_wheel_gesture() {
    let mut bytes = Vec::new();
    write_apple_scroll(&mut bytes, AppleScroll { x: 10, y: 20, right: 0, down: 100 }).unwrap();

    assert_eq!(bytes.len(), 36 + 58, "a compact scroll and a scroll wheel event go together");
    let lines = (100.0f64 / 45.0) as f32;
    let mut compact = vec![0x17, 0, 0, 32, 0, 2, 0, 8];
    compact.extend_from_slice(&0.0f32.to_be_bytes());
    compact.extend_from_slice(&lines.to_be_bytes());
    compact.extend_from_slice(&0.0f32.to_be_bytes());
    compact.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 20]);
    assert_eq!(bytes[..36], compact[..], "sub-type 8 carries the lines as floats, one scroll, no phases and the point");
    let mut wheel = vec![0x17, 0, 0, 54, 0, 1, 0, 11];
    wheel.extend_from_slice(&[0, 0, 0, 2, 0, 0]);
    wheel.extend_from_slice(&[0, 0, 0, 0, 0, 2, 0x38, 0xe4, 0, 0, 0, 0]);
    wheel.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0]);
    wheel.extend_from_slice(&[0; 16]);
    wheel.extend_from_slice(&[0, 10, 0, 20]);
    assert_eq!(bytes[36..], wheel[..], "sub-type 11 carries whole lines, 16.16 lines, pixels, no phases or flags and the point");

    let mut reader = &bytes[..];
    let first = ClientMessage::read(&mut reader).unwrap();
    let second = ClientMessage::read(&mut reader).unwrap();
    assert_eq!(
        first,
        ClientMessage::AppleGesture { version: 2, kind: 8, data: bytes[8..36].to_vec() },
        "the server side reads the compact scroll as a gesture"
    );
    assert_eq!(
        second,
        ClientMessage::AppleGesture { version: 1, kind: 11, data: bytes[44..].to_vec() },
        "the server side reads the scroll wheel event as a gesture"
    );
}
