use axum::Json;
use axum::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::SystemTime;

// handler accepts the payload directly using Axum's extractors
pub async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
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
    let claims = Claims {
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

    // return the response as JSON
    Ok(Json(LoginResponse {
        access_token: token,
        token_type: String::from("Bearer"),
    }))
}

// struct representing the incoming request body
#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

// struct representing the JSON response body
#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
    token_type: String,
}

// claims that will be encoded inside the JWT
#[derive(Serialize)]
pub struct Claims {
    sub: String, // subject (usually email)
    exp: usize,  // expiration timestamp
}
