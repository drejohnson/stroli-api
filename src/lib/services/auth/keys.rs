use std::sync::OnceLock;

use crate::domain::secrets::AppSecrets;

pub struct Keys {
    pub private: String,
    pub public: String,
}

static KEYS: OnceLock<Keys> = OnceLock::new();

pub fn get_keys(secrets: &AppSecrets) -> &'static Keys {
    KEYS.get_or_init(|| {
        let private = &secrets.jwt_rs256_private;
        let public = &secrets.jwt_rs256_public;
        Keys {
            private: private.clone(),
            public: public.clone(),
        }
    })
}
