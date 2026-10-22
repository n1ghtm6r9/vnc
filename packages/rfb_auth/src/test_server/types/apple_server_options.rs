use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct AppleServerOptions {
    pub generator: Vec<u8>,
    pub prime: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl fmt::Debug for AppleServerOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppleServerOptions")
            .field("generator", &self.generator)
            .field("prime", &self.prime)
            .field("private_key", &"***")
            .finish()
    }
}
