pub mod error;
pub mod routes;

use std::{sync::{Arc, Mutex}, vec};
use tera::{Context, Tera};
use tokio::sync::oneshot;

// Shared application state
pub struct AppState {
    pub tera: Tera,
    pub context: Mutex<Context>,
}

impl AppState {
    pub fn new(templates_dir: &str) -> Self {
        let tera = Tera::new(templates_dir).expect("Failed to initialize Tera");
        let mut context = Context::new();
        init_test_context(&mut context);
        Self {
            tera,
            context: Mutex::new(context),
        }
    }
}

pub async fn create_and_run_webserver(template_dir: &str, addr: &str, shutdown_rx: oneshot::Receiver<()>) {
    let app_state = Arc::new(AppState::new(template_dir));
    let app = routes::create_router(app_state);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        }).await.unwrap();
}

pub fn init_test_context(context: &mut Context) {
    context.insert("teams", &vec!["Team 1", "Team 2", "Team 3", "Team 4", "Team 5", "Team 6", "Team 7", "Team 8"]);
    context.insert("matchups", &vec![
        vec!["Team 1", "Team 2", "Team 3", "Team 4", "Team 5", "Team 6", "Team 7", "Team 8"],
        vec!["Quarterfinalist 1", "Quarterfinalist 2", "Quarterfinalist 3", "Quarterfinalist 4"],
        vec!["Semifinalist 1", "Semifinalist 2"],
        vec!["Winner"],
    ]);
}
