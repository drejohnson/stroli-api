use std::sync::OnceLock;

use shuttle_runtime::SecretStore;

#[derive(Debug, Clone)]
pub struct AppSecrets {
    pub aws_url: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub db_endpoint: String,
    pub db_username: String,
    pub db_password: String,
    pub db_namespace: String,
    pub db_database_name: String,
    pub jwt_name: String,
    pub jwt_hs512_secret: String,
    pub jwt_rs256_public: String,
    pub jwt_rs256_private: String,
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

pub fn grab_secrets(secrets: SecretStore) -> &'static AppSecrets {
    static APP_SECRETS: OnceLock<AppSecrets> = OnceLock::new();

    APP_SECRETS.get_or_init(|| AppSecrets {
        aws_url: secrets.get("AWS_URL").unwrap_or_else(|| "None".to_string()),
        aws_access_key_id: secrets
            .get("AWS_ACCESS_KEY_ID")
            .unwrap_or_else(|| "None".to_string()),
        aws_secret_access_key: secrets
            .get("AWS_SECRET_ACCESS_KEY")
            .unwrap_or_else(|| "None".to_string()),
        aws_region: secrets
            .get("AWS_REGION")
            .unwrap_or_else(|| "None".to_string()),
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
        jwt_hs512_secret: secrets
            .get("JWT_HS512_SECRET")
            .unwrap_or_else(|| "None".to_string()),
        jwt_rs256_public: secrets
            .get("JWT_RS256_PUBLIC")
            .unwrap_or_else(|| "None".to_string()),
        jwt_rs256_private: secrets
            .get("JWT_RS256_PRIVATE")
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
    })
}
