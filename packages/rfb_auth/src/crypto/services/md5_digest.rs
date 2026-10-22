use md5::{Digest, Md5};

pub fn md5_digest(data: &[u8]) -> [u8; 16] {
    Md5::digest(data).into()
}
