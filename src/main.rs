// modules in project-scope
mod auth;

// routers to be nested into the main router
use auth::router::auth_router;

use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    // load environment variables from the .env file
    dotenvy::dotenv().expect("Failed to read .env file.");

    // retrieve the SERVER_PORT environment variable; default to 3000
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());

    // run our app with hyper, listening globally on specified port
    let listener_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&listener_addr)
        .await
        .expect("Failed to bind to the specified port.");

    // print that the application has bound
    println!("\nListening on: {}", listener_addr);

    // fetch environment variables related to the PostgreSQL instance
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set.");
    let database_username = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set.");
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set.");

    // build the database url from the variables fetched above
    let database_url = format!(
        "postgres://{}:{}@localhost:5432/{}",
        database_username, database_password, database_name
    );

    // create the Postgres connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres.");

    // print that the database pool has been created
    println!("PostgreSQL database pool successfully created!");

    // create a new AppState instance
    let state = AppState {
        db_pool: db_pool.clone(),
    };

    // serve the application with axum
    axum::serve(listener, build_app(state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    // server has stopped receiving new requests
    println!("\nServer stopped. Closing database pool safely...");

    // Explicitly drain and close all connections cleanly
    db_pool.close().await;

    // server program end-of-life
    println!("Database disconnected. Goodbye!");
}

// builds the application router
fn build_app(app_state: AppState) -> axum::Router {
    axum::Router::new()
        // attach the auth routes with prefix /api/auth/
        .nest("/api/auth/", auth_router())
        // hoist global state for all requests
        .with_state(app_state)
}

// global application state
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}

// define a signal for graceful shutdown
async fn shutdown_signal() {
    // wait for Ctrl+C or SIGTERM
    tokio::signal::ctrl_c().await.unwrap();
}
