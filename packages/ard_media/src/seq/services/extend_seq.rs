use super::seq_delta;

pub fn extend_seq(reference: i64, seq: u16) -> i64 {
    reference + seq_delta(i64::from(seq), reference)
}
