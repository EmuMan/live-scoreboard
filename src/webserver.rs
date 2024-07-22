pub mod error;
pub mod routes;

use std::sync::{Arc, Mutex};
use tera::{Context, Tera};
use tokio::sync::oneshot;

use crate::SharedState;

// Shared application state
pub struct WebserverState {
    pub tera: Tera,
    pub context: Mutex<Context>,
    pub shared_state: SharedState,
}

impl WebserverState {
    pub fn new(templates_dir: &str, shared_state: SharedState) -> Self {
        let tera = Tera::new(templates_dir).expect("Failed to initialize Tera");
        let context = Context::new();
        Self {
            tera,
            context: Mutex::new(context),
            shared_state,
        }
    }
}

pub async fn create_and_run_webserver(
    template_dir: &str,
    addr: &str,
    shutdown_rx: oneshot::Receiver<()>,
    shared_state: SharedState,
) {
    let webserver_state = Arc::new(WebserverState::new(template_dir, shared_state));
    let app = routes::create_router(webserver_state);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        }).await.unwrap();
}
