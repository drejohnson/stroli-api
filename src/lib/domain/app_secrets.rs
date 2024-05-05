#[derive(Debug, Clone)]
pub struct AppSecrets {
    pub edgedb_instance: String,
    pub edgedb_secret_key: String,
    pub stripe_key: String,
    pub stripe_sub_price: String,
    pub mailgun_key: String,
    pub mailgun_url: String,
    pub domain: String,
}

pub fn grab_secrets(secrets: shuttle_runtime::SecretStore) -> AppSecrets {
    AppSecrets {
        edgedb_instance: secrets
            .get("EDGEDB_INSTANCE")
            .unwrap_or_else(|| "None".to_string()),
        edgedb_secret_key: secrets
            .get("EDGE_DB_SECRET_KEY")
            .unwrap_or_else(|| "None".to_string()),
        stripe_key: secrets
            .get("STRIPE_KEY")
            .unwrap_or_else(|| "None".to_string()),
        stripe_sub_price: secrets
            .get("STRIPE_SUB_PRICE")
            .unwrap_or_else(|| "None".to_string()),
        mailgun_key: secrets
            .get("MAILGUN_KEY")
            .unwrap_or_else(|| "None".to_string()),
        mailgun_url: secrets
            .get("MAILGUN_URL")
            .unwrap_or_else(|| "None".to_string()),
        domain: secrets
            .get("DOMAIN_URL")
            .unwrap_or_else(|| "None".to_string()),
    }
}
