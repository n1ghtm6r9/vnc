use crate::{APPLE_HIGH_PERFORMANCE_BITS, APPLE_HIGH_PERFORMANCE_BYTE};

pub fn apple_offers_high_performance(capabilities: &[u8; 16]) -> bool {
    capabilities[APPLE_HIGH_PERFORMANCE_BYTE] & APPLE_HIGH_PERFORMANCE_BITS == APPLE_HIGH_PERFORMANCE_BITS
}
