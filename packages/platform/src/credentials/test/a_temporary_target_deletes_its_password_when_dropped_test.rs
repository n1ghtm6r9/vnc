use crate::test_support::TemporaryTarget;
use crate::{load_password, save_password};

#[test]
fn test_a_temporary_target_deletes_its_password_when_dropped() {
    let target = TemporaryTarget::new("dropped");
    let name = target.name().to_owned();
    save_password(&name, "bogdan", "secret").expect("the credential store accepts a new password");
    drop(target);
    assert_eq!(load_password(&name), Ok(None), "a dropped temporary target leaves no password behind");
}
