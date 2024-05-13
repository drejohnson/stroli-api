use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

use crate::auth::utils::generate_secret_key;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret_key = generate_secret_key(32);
    Keys::new(secret_key.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
