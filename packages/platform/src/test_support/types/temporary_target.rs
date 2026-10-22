use crate::delete_password;

pub struct TemporaryTarget {
    pub(super) name: String,
}

impl Drop for TemporaryTarget {
    fn drop(&mut self) {
        if let Err(error) = delete_password(&self.name) {
            println!("the test password {} could not be deleted: {error}", self.name);
        }
    }
}
