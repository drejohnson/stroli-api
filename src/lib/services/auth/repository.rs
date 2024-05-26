use axum::async_trait;

use surrealdb::engine::any::Any;
use surrealdb::opt::auth::{Jwt, Scope};
use surrealdb::Surreal;

use crate::services::auth::error::Result;
use crate::services::auth::model::{SigninParams, SignupParams};

#[async_trait]
pub trait AuthRepository {
    fn new(db: &Surreal<Any>) -> Self;
    async fn signup_user(&self, user: SignupParams) -> Result<Jwt>;
    async fn signin_user(&self, user: SigninParams) -> Result<Jwt>;
    // async fn verify_user(&self, user: SigninParams) -> Result<bool>;
    // async fn get_user_by_email(&self, email: &str) -> Result<Option<SignupParams>>;
}

pub struct AuthRepositoryImpl {
    pub db: Surreal<Any>,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    fn new(db: &Surreal<Any>) -> Self {
        Self { db: db.clone() }
    }
    async fn signup_user(&self, user: SignupParams) -> Result<Jwt> {
        let response = self
            .db
            .signup(Scope {
                namespace: "stroli",
                database: "app_stroli",
                scope: "user",
                params: user,
            })
            .await?;
        // let token = response.as_insecure_token();
        Ok(response)
    }

    async fn signin_user(&self, user: SigninParams) -> Result<Jwt> {
        let response = self
            .db
            .signin(Scope {
                namespace: "stroli",
                database: "app_stroli",
                scope: "user",
                params: user,
            })
            .await?;
        Ok(response)
    }

    // async fn verify_user(&self, user: SigninParams) -> Result<bool> {
    //     // Implementation to verify user credentials
    //     Ok(true)
    // }

    // async fn get_user_by_email(&self, email: &str) -> Result<Option<SignupParams>> {
    //     // Implementation to get user by email from the database
    //     Ok(Some(SignupParams {
    //         name: "example".into(),
    //         email: email.into(),
    //         password: "hashed_password".into(),
    //     }))
    // }
}
