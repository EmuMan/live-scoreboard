pub mod error;
pub mod routes;

use std::{collections::HashMap, sync::{Arc, Mutex}};
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
        let mut tera = Tera::new(templates_dir).expect("Failed to initialize Tera");
        tera.register_filter("max", tera_max);
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

fn tera_max(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    println!("value: {:?}", value);
    let as_array = value.as_array().ok_or_else(|| {
        tera::Error::msg("value is not an array")
    })?;
    let mut as_numbers: Vec<i64> = Vec::new();
    for v in as_array {
        match v {
            tera::Value::Number(n) => {
                as_numbers.push(n.as_i64().ok_or_else(|| {
                    tera::Error::msg("value is not an integer")
                })?);
            },
            _ => return Err(tera::Error::msg("value is not an integer")),
        }
    };
    let max = as_numbers.iter().max().ok_or_else(|| {
        tera::Error::msg("no maximum value found")
    })?;
    Ok(tera::Value::Number(tera::Number::from(*max)))
}
