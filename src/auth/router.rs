use crate::AppState;
use crate::auth::handlers::login::login;
use axum::Router;
use axum::routing::post;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        // login route that validates credentials and returns a JWT
        .route("/login", post(login))
}
