use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Runtime;

pub mod webserver;
pub mod ui;
pub mod models;

#[derive(Clone)]
pub struct AppState {
    pub value: i32,
}

type SharedState = Arc<Mutex<AppState>>;

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}
