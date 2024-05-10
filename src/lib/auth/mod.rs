pub mod handlers;
pub mod models;
pub mod pkce;
pub mod service;
pub mod utils;

pub use handlers::{login_handler, signup_handler};
pub use models::{
    AuthDetails, AuthenticateRequest, AuthenticateResponse, HandleAuthorize, Signup,
    UserCredentials,
};
pub use pkce::Pkce;
pub use service::AuthService;
pub use utils::{handle_email_password_signin, handle_email_password_signup};
