use crate::domain::secrets::AppSecrets;

use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::Client as S3Client;

pub async fn initialize_aws_s3_client(secrets: &AppSecrets) -> S3Client {
    let creds = Credentials::from_keys(
        &secrets.aws_access_key_id,
        &secrets.aws_secret_access_key,
        None,
    );

    let cfg = aws_config::from_env()
        .endpoint_url(&secrets.aws_url)
        .region(Region::new("eu-east-2"))
        .credentials_provider(creds)
        .load()
        .await;

    S3Client::new(&cfg)
}
