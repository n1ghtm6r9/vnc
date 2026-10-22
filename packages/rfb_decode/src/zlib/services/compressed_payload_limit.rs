use crate::PAYLOAD_SLACK;

pub(crate) fn compressed_payload_limit(inflated_limit: usize) -> usize {
    inflated_limit
        .saturating_add(inflated_limit / 8)
        .saturating_add(PAYLOAD_SLACK)
}
