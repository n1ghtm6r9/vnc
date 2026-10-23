use crate::KIND_ANSWER;

pub fn is_answer(payload: &[u8]) -> bool {
    payload.get(2..4) == Some(KIND_ANSWER.to_be_bytes().as_slice())
}
