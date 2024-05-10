use std::sync::Arc;

// dependencies
use axum::{extract::Request, Router, ServiceExt};
use edgedb_tokio::{Builder, Client, GlobalsDelta};
use reqwest::Client as ReqwestClient;
use stroli_api_lib::auth::AuthService;
use stroli_api_lib::domain::{grab_secrets, AppSecrets, AppState};
use stroli_api_lib::errors::ApiError;
use stroli_api_lib::startup::build_api_router;
use tower_http::normalize_path::NormalizePathLayer;

#[derive(GlobalsDelta)]
struct Globals<'a> {
    #[edgedb(rename = "ext::auth::client_token")]
    auth_token: &'a str,
}

// type to wrap a router and implement the Service trait for it
struct StroliService {
    app_router: Router,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for StroliService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        axum::serve(
            tokio::net::TcpListener::bind(addr).await?,
            ServiceExt::<Request>::into_make_service(
                self.app_router
                    .layer(NormalizePathLayer::trim_trailing_slash()),
            ),
        )
        .await?;

        Ok(())
    }
}

// main function, annotated for Shuttle
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: shuttle_runtime::SecretStore,
) -> Result<StroliService, shuttle_runtime::Error> {
    let secrets = grab_secrets(secret_store);

    let http_client = Arc::new(ReqwestClient::new());
    let auth_service = AuthService::new(http_client, secrets.edgedb_base_auth_url.clone());

    // Create a client using the secrets
    let conn = create_client(secrets.clone()).await?;

    let app_state = AppState::initialize(conn, secrets);

    // intialize the app state with the database pool
    let shared_state = AppState::initialize_shared_state(app_state);

    // create an Axum router
    let app_router = build_api_router(shared_state, auth_service);

    // return a StroliService instance
    Ok(StroliService { app_router })
}

async fn create_client(secrets: AppSecrets) -> Result<Client, ApiError> {
    let token_from_auth_server: String = String::from("your_token_here");
    let cfg = Builder::new()
        .instance(&secrets.edgedb_instance)?
        .secret_key(&secrets.edgedb_secret_key)
        .build_env()
        .await
        .map_err(|e| ApiError::Database(e))?;

    let pool = Client::new(&cfg);

    pool.with_globals(&Globals {
        auth_token: token_from_auth_server.as_str(),
    });

    pool.ensure_connected()
        .await
        .map_err(|e| ApiError::Database(e))?;

    Ok(pool)
}
