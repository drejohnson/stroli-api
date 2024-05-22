use axum::{extract::Request, Router, ServiceExt};
use shuttle_runtime::DeploymentMetadata;
use stroli_api_lib::{
    domain::{secrets::grab_secrets, state::AppState},
    infrastructure::{db::connect_to_database, s3::initialize_aws_s3_client},
    startup::build_api_router,
};
use tower_http::normalize_path::NormalizePathLayer;
use tower_layer::Layer;

pub type Result<T> = core::result::Result<T, shuttle_runtime::Error>;

struct StroliService {
    app_router: Router,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for StroliService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<()> {
        let router = self.app_router;
        let router = NormalizePathLayer::trim_trailing_slash().layer(router);

        axum::serve(
            tokio::net::TcpListener::bind(addr).await?,
            ServiceExt::<Request>::into_make_service(router),
        )
        .await?;

        Ok(())
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: shuttle_runtime::SecretStore,
    #[shuttle_runtime::Metadata] _metadata: DeploymentMetadata,
) -> Result<StroliService> {
    let secrets = grab_secrets(secret_store);

    // Create a client using the secrets
    let db = connect_to_database(&secrets).await?;
    let s3 = initialize_aws_s3_client(&secrets).await;

    let app_state = AppState::initialize(&db, &s3, secrets);

    // intialize the app state with the database pool
    let shared_state = AppState::initialize_shared_state(app_state);

    let app_router =
        build_api_router(shared_state).map_err(stroli_api_lib::startup::Error::from)?;

    // return a StroliService instance
    Ok(StroliService { app_router })
}
