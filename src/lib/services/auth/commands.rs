use surrealdb::opt::auth::Jwt;

use crate::domain::state::SharedState;

use crate::services::auth::error::Result;
use crate::services::auth::model::{SigninParams, SignupParams};
use crate::services::auth::repository::{AuthRepository, AuthRepositoryImpl};
use crate::services::auth::service::AuthService;

pub async fn register_user(state: &SharedState, user: SignupParams) -> Result<Jwt> {
    let service = AuthService::new(AuthRepositoryImpl::new(&state.db));
    service.register_user(user).await
}

pub async fn login_user(state: &SharedState, user: SigninParams) -> Result<Jwt> {
    let service = AuthService::new(AuthRepositoryImpl::new(&state.db));
    service.login_user(user).await
}
