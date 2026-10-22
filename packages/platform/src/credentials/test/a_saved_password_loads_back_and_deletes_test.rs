use crate::test_support::TemporaryTarget;
use crate::{delete_password, load_password, save_password};

#[test]
fn test_a_saved_password_loads_back_and_deletes() {
    let temporary = TemporaryTarget::new("round-trip");
    let target = temporary.name();
    let password = "пароль с пробелом и 🔑";
    save_password(target, "bogdan", password).expect("the credential store accepts a new password");
    let loaded = load_password(target).expect("the credential store answers");
    if cfg!(windows) {
        assert_eq!(loaded.as_deref(), Some(password), "the saved password comes back unchanged, Cyrillic and emoji included");
    }
    delete_password(target).expect("the saved password can be deleted");
    assert_eq!(load_password(target).expect("the credential store answers"), None, "a deleted password is gone");
}
