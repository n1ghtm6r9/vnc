use std::time::Instant;

use super::{message1_payload, test_answer, test_session};
use crate::{MediaSession, test_support::FakeDecoder};

pub fn answered_session(decoder: FakeDecoder, start: Instant) -> MediaSession<FakeDecoder> {
    let mut session = test_session(decoder, start);
    session.handle_message1(&message1_payload(), start);
    session.handle_answer(&test_answer(), start);
    session.effects.take();
    session
}
