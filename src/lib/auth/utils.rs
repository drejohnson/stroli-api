use rand::{distributions::Alphanumeric, Rng};

pub fn generate_secret_key(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
