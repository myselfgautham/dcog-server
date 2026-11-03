use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::SystemTime;

// handler accepts the payload directly using Axum's extractors
pub async fn email_login(
    cookie_jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // TODO: Add database verification
    if payload.email != "someone@example.com" || payload.password != "password123" {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // fetch the JWT secret from environment variables
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // set the token's expiration time
    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 3600; // 3600 seconds = 1 hour

    // define the claims for the token
    let claims = JwtClaims {
        sub: payload.email,
        exp: expiration as usize,
    };

    // encode the token using the secret key
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // build the HTTP-only cookie
    let cookie = Cookie::build(("session_token", token))
        .http_only(true) // prevents client-side JS from accessing the cookie
        .secure(true) // ensures it is only sent over HTTPS
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    // add the cookie to jar and return it along with response tuple
    Ok((cookie_jar.add(cookie), "Logged in successfully!"))
}

// struct representing the incoming request body
#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

// claims that will be encoded inside the JWT
#[derive(Serialize)]
pub struct JwtClaims {
    sub: String, // subject (usually email)
    exp: usize,  // expiration timestamp
}
