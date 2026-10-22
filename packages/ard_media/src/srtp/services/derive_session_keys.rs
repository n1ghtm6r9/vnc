use aes::{Aes256, cipher::KeyInit};

use super::derive_key;
use crate::{MasterKey, SessionKeys, SrtpKeys};

pub fn derive_session_keys(master: &MasterKey) -> SessionKeys {
    let cipher = Aes256::new((&master.key).into());
    let salt = &master.salt;
    SessionKeys {
        rtp: SrtpKeys {
            cipher_key: derive_key(&cipher, salt, 0),
            auth_key: derive_key(&cipher, salt, 1),
            salt: derive_key(&cipher, salt, 2),
        },
        rtcp: SrtpKeys {
            cipher_key: derive_key(&cipher, salt, 3),
            auth_key: derive_key(&cipher, salt, 4),
            salt: derive_key(&cipher, salt, 5),
        },
    }
}
