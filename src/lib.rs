use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};

pub mod webserver;
pub mod ui;
pub mod models;
pub mod fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub division: models::Division,
    pub assets: Vec<models::Asset>,
}

type SharedState = Arc<Mutex<AppState>>;

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}
