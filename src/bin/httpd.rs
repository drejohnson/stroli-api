// dependencies
use axum::{extract::Request, Router, ServiceExt};
use edgedb_tokio::Builder;
use edgedb_tokio::Client;
use shuttle_runtime::Error;
use stroli_api_lib::domain::grab_secrets;
use stroli_api_lib::domain::AppSecrets;
use stroli_api_lib::domain::AppState;
use stroli_api_lib::startup::build_api_router;
use tower_http::normalize_path::NormalizePathLayer;
use tower_layer::Layer;

// type to wrap a router and implement the Service trait for it
struct StroliService {
    app_router: Router,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for StroliService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<(), Error> {
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

// main function, annotated for Shuttle
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: shuttle_runtime::SecretStore,
) -> Result<StroliService, Error> {
    let secrets = grab_secrets(secret_store);

    let conn = create_client(secrets.clone())
        .await
        .expect("Client should have initiated");

    // initialize the application state to hold the database pool
    let app_state = AppState::initialize(conn, secrets);

    // intialize the app state with the database pool
    let shared_state = AppState::initialize_shared_state(app_state);

    // create an Axum router
    let app_router = build_api_router(shared_state);

    // return a StroliService instance
    Ok(StroliService { app_router })
}

pub async fn create_client(secrets: AppSecrets) -> Result<Client, Error> {
    let cfg = Builder::new()
        .instance(&secrets.edgedb_instance)
        .unwrap()
        .secret_key(&secrets.edgedb_secret_key)
        .build_env()
        .await
        .unwrap();
    let pool = Client::new(&cfg);
    pool.ensure_connected().await.unwrap();
    Ok(pool)
}
