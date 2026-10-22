use std::time::Instant;

use crate::{
    MediaSession,
    test_support::{FakeDecoder, fixed_secrets},
};

pub fn test_session(decoder: FakeDecoder, start: Instant) -> MediaSession<FakeDecoder> {
    MediaSession::new(decoder, fixed_secrets, start)
}
