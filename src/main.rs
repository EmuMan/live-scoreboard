use axum::{
    extract::Extension,
    routing::{get, Router}
};
use std::sync::{Arc, Mutex};
use tera::{Context, Tera};

#[tokio::main]
async fn main() {
    // Initialize Tera template engine
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    // Initialize the context with some data
    let mut context = Context::new();
    live_scoreboard::init_test_context(&mut context);

    // Shared application state wrapped in Arc and Mutex
    let app_state = Arc::new(live_scoreboard::AppState {
        tera,
        context: Mutex::new(context),
    });

    // Create the Axum application
    let app = Router::new()
        .route("/scoreboard", get(live_scoreboard::render_scoreboard))
        .route("/assets/*path", get(live_scoreboard::serve_asset))
        .layer(Extension(app_state));

    // Run the application
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
