#[derive(Debug, Clone)]
pub struct AppSecrets {
    pub db_endpoint: String,
    pub db_username: String,
    pub db_password: String,
    pub db_namespace: String,
    pub db_database_name: String,
    pub jwt_name: String,
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub kafka_url: String,
    pub kafka_sasl_user: String,
    pub kafka_sasl_pass: String,
    pub stripe_key: String,
    pub stripe_sub_price: String,
    pub mailgun_key: String,
    pub mailgun_url: String,
    pub domain: String,
}

pub fn grab_secrets(secrets: shuttle_runtime::SecretStore) -> AppSecrets {
    AppSecrets {
        db_endpoint: secrets
            .get("SURREALDB_URL")
            .unwrap_or_else(|| "None".to_string()),
        db_username: secrets
            .get("SURREALDB_USERNAME")
            .unwrap_or_else(|| "None".to_string()),
        db_password: secrets
            .get("SURREALDB_PASSWORD")
            .unwrap_or_else(|| "None".to_string()),
        db_namespace: secrets
            .get("SURREALDB_NAMESPACE")
            .unwrap_or_else(|| "None".to_string()),
        db_database_name: secrets
            .get("SURREALDB_DATABASE_NAME")
            .unwrap_or_else(|| "None".to_string()),
        jwt_name: secrets
            .get("JWT_NAME")
            .unwrap_or_else(|| "None".to_string()),
        jwt_secret: secrets
            .get("JWT_SECRET")
            .unwrap_or_else(|| "None".to_string()),
        jwt_issuer: secrets
            .get("JWT_ISSUER")
            .unwrap_or_else(|| "None".to_string()),
        kafka_url: secrets
            .get("KAFKA_URL")
            .unwrap_or_else(|| "None".to_string()),
        kafka_sasl_user: secrets
            .get("KAFKA_SASL_USER")
            .unwrap_or_else(|| "None".to_string()),
        kafka_sasl_pass: secrets
            .get("KAFKA_SASL_PASS")
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
