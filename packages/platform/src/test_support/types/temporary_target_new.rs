use super::TemporaryTarget;
use crate::test_support::unique_target;

impl TemporaryTarget {
    pub fn new(label: &str) -> TemporaryTarget {
        TemporaryTarget {
            name: unique_target(label),
        }
    }
}
