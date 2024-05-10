use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use rand::Rng;
use sha2::{Digest, Sha256};

pub struct Pkce {
    pub verifier: String,
    pub challenge: String,
}

impl Pkce {
    pub fn generate() -> Self {
        let random_bytes: [u8; 32] = rand::thread_rng().gen();
        const CUSTOM_ENGINE: engine::GeneralPurpose =
            engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
        let verifier = CUSTOM_ENGINE.encode(&random_bytes);

        let mut hasher = Sha256::new();
        hasher.update(&random_bytes);
        let challenge_bytes = hasher.finalize();
        let challenge = CUSTOM_ENGINE.encode(&challenge_bytes);

        Self {
            verifier,
            challenge,
        }
    }
}
