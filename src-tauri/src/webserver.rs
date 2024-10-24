pub mod error;
pub mod routes;

use std::{collections::HashMap, sync::Arc};
use tera::Tera;
use tokio::sync::oneshot;

use crate::SharedState;

pub fn tera_is_null(args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    match args.get("value") {
        Some(value) => Ok(serde_json::Value::Bool(value.is_null())),
        None => Err("No value provided".into()),
    }
}

// Shared application state
pub struct WebserverState {
    pub tera: Tera,
    pub shared_state: SharedState,
}

impl WebserverState {
    pub fn new(templates_dir: &str, shared_state: SharedState) -> Self {
        let mut tera = Tera::new(templates_dir)
            .expect("Failed to initialize Tera");
        tera.register_function("is_null", tera_is_null);
        Self {
            tera,
            shared_state,
        }
    }
}

pub async fn create_and_run_webserver(
    template_dir: String,
    addr: &str,
    shutdown_rx: oneshot::Receiver<()>,
    shared_state: SharedState,
) {
    let webserver_state = Arc::new(WebserverState::new(&template_dir, shared_state));
    let app = routes::create_router(webserver_state);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        }).await.unwrap();
}
