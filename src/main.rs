use axum::routing::get;
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
    println!("Listening on: {}", listener_addr);

    // serve the application with axum
    axum::serve(listener, build_app()).await.unwrap();
}

// builds the application router
fn build_app() -> axum::Router {
    axum::Router::new()
        // TODO: replace; hello, world route for testing
        .route("/", get(|| async { "Hello, World!" }))
}
