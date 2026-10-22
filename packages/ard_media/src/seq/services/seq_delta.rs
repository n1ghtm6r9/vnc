use crate::{SEQ_HALF, SEQ_MODULUS};

pub fn seq_delta(to: i64, from: i64) -> i64 {
    (to - from + SEQ_HALF).rem_euclid(SEQ_MODULUS) - SEQ_HALF
}
