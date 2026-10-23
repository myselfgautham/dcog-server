use crate::AppState;
use crate::auth::handlers::email_login::email_login;
use axum::Router;
use axum::routing::post;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        // login route that validates credentials and returns a JWT
        .route("/login/email", post(email_login))
}
