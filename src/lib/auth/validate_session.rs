use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::PrivateCookieJar;
use http::StatusCode;

use crate::domain::AppState;

// Create a placeholder function for the validate_session function that will return void
pub async fn validate_session(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> (PrivateCookieJar, Response) {
    (jar, (StatusCode::OK, "OK".to_string()).into_response())
}

// pub async fn validate_session(
//     jar: PrivateCookieJar,
//     State(state): State<AppState>,
//     request: Request,
//     next: Next,
// ) -> (PrivateCookieJar, Response) {
//     let Some(cookie) = jar.get("foo").map(|cookie| cookie.value().to_owned()) else {
//         println!("Couldn't find a cookie in the jar");
//         return (
//             jar,
//             (StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response(),
//         );
//     };

//     let find_session = state
//         .session_store
//         .read()
//         .unwrap()
//         .get(&cookie)
//         .map(|session| session.clone());

//     match find_session {
//         Some(session) => {
//             let (jar, response) = next.run(request).await;
//             (jar, response)
//         }
//         None => (
//             jar,
//             (StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response(),
//         ),
//     }
// }
