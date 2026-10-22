use crate::test_support::unique_target;
use crate::{delete_password, load_password};

#[test]
fn test_a_missing_target_loads_as_none() {
    let target = unique_target("missing");
    assert_eq!(load_password(&target), Ok(None), "a target that was never saved has no password");
    assert_eq!(delete_password(&target), Ok(()), "deleting a missing target is not an error");
}
