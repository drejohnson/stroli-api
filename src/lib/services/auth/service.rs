use surrealdb::opt::auth::Jwt;

use crate::services::auth::error::Result;
use crate::services::auth::model::{SigninParams, SignupParams};
use crate::services::auth::repository::AuthRepository;

pub struct AuthService<R: AuthRepository> {
    repository: R,
}

impl<R: AuthRepository> AuthService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn register_user(&self, user: SignupParams) -> Result<Jwt> {
        self.repository.signup_user(user).await
    }

    pub async fn login_user(&self, user: SigninParams) -> Result<Jwt> {
        self.repository.signin_user(user).await
    }
}
