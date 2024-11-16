pub mod webserver;
pub mod models;
pub mod fs;

use std::sync::{Arc, Mutex};
use tokio::{self, sync::oneshot::{self, Sender}};
use tauri::{State, Builder, Manager, async_runtime::JoinHandle};

use models::{SaveData, Settings, Division, Resources, Match};

#[derive(Debug)]
pub struct AppState {
    pub loaded_config: Option<std::path::PathBuf>,
    pub webserver_handle: Option<JoinHandle<()>>,
    pub webserver_stop_tx: Option<Sender<()>>,
    pub data: SaveData,
}

type SharedState = Arc<Mutex<AppState>>;

impl AppState {
    pub fn new(
        loaded_config: Option<std::path::PathBuf>,
        webserver_handle: Option<JoinHandle<()>>,
        webserver_stop_tx: Option<Sender<()>>,
        data: SaveData,
    ) -> Self {
        Self {
            loaded_config,
            webserver_handle,
            webserver_stop_tx,
            data,
        }
    }

    pub fn new_shared(
        loaded_config: Option<std::path::PathBuf>,
        webserver_handle: Option<JoinHandle<()>>,
        webserver_stop_tx: Option<Sender<()>>,
        data: SaveData,
    ) -> SharedState {
        Arc::new(Mutex::new(AppState::new(
            loaded_config,
            webserver_handle,
            webserver_stop_tx,
            data,
        )))
    }

    pub fn get_base_path(&self) -> Option<std::path::PathBuf> {
        self.loaded_config.as_ref()
            .map(|path| fs::remove_file_from_path(path))
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(
            None,
            None,
            None,
            SaveData::default(),
        )
    }
}

#[tauri::command]
fn start_webserver(shared_state: State<'_, SharedState>) -> bool {
    let Some(base_path) = shared_state.lock().unwrap().get_base_path() else {
        println!("Failed to start webserver: no config loaded!");
        return false;
    };
    let templates_path = fs::from_relative_path(&base_path, "templates/**/*");
    let (tx, rx) = oneshot::channel::<()>();
    let handle = tauri::async_runtime::spawn(
        webserver::create_and_run_webserver(
            templates_path,
            "0.0.0.0:3000",
            rx,
            shared_state.inner().clone()
        )
    );
    {
        let mut state = shared_state.lock().unwrap();
        state.webserver_handle = Some(handle);
        state.webserver_stop_tx = Some(tx);
    }
    true
}

#[tauri::command]
fn stop_webserver(shared_state: State<'_, SharedState>) -> bool {
    let mut state = shared_state.lock().unwrap();
    if let Some(tx) = state.webserver_stop_tx.take() {
        return tx.send(()).is_ok()
    }
    true
}

#[tauri::command]
fn get_loaded_config(shared_state: State<'_, SharedState>) -> Option<String> {
    let state = shared_state.lock().unwrap();
    state.loaded_config.as_ref()
        .map(|path| path.to_string_lossy().to_string())
}

#[tauri::command]
fn get_settings(shared_state: State<'_, SharedState>) -> Settings {
    let state = shared_state.lock().unwrap();
    state.data.settings.clone()
}

#[tauri::command]
fn set_settings(shared_state: State<'_, SharedState>, settings: Settings) {
    let mut state = shared_state.lock().unwrap();
    state.data.settings = settings;
}

#[tauri::command]
fn get_division(shared_state: State<'_, SharedState>) -> Division {
    let state = shared_state.lock().unwrap();
    state.data.division.clone()
}

#[tauri::command]
fn set_division(shared_state: State<'_, SharedState>, division: Division) {
    let mut state = shared_state.lock().unwrap();
    state.data.division = division;
}

#[tauri::command]
fn get_resources(shared_state: State<'_, SharedState>) -> Resources {
    let state = shared_state.lock().unwrap();
    state.data.resources.clone()
}

#[tauri::command]
fn set_resources(shared_state: State<'_, SharedState>, resources: Resources) {
    let mut state = shared_state.lock().unwrap();
    state.data.resources = resources;
}

#[tauri::command]
fn get_current_match(shared_state: State<'_, SharedState>) -> Match {
    let state = shared_state.lock().unwrap();
    state.data.current_match.clone()
}

#[tauri::command]
fn set_current_match(shared_state: State<'_, SharedState>, current_match: Match) {
    let mut state = shared_state.lock().unwrap();
    state.data.current_match = current_match;
}

#[tauri::command]
fn load_from_filename(shared_state: State<'_, SharedState>, filename: String) -> bool {
    let path = std::path::Path::new(&filename);
    let mut state = shared_state.lock().unwrap();
    fs::read_into_state_from_config_file(&mut state, path);
    true
}

#[tauri::command]
fn save_to_filename(shared_state: State<'_, SharedState>, filename: String) -> bool {
    let path = std::path::Path::new(&filename);
    let mut state = shared_state.lock().unwrap();
    fs::write_state_to_config_file(&mut state, path);
    true
}

#[tauri::command]
fn to_relative_path(shared_state: State<'_, SharedState>, path: String) -> Option<String> {
    let base_path = shared_state.lock().unwrap().get_base_path()?;
    fs::to_relative_path(&base_path, &std::path::Path::new(&path))
}

#[tauri::command]
fn from_relative_path(shared_state: State<'_, SharedState>, path: String) -> Option<String> {
    shared_state.lock().unwrap().get_base_path().map(|base_path| {
        fs::from_relative_path(&base_path, &path)
    })
}

#[tauri::command]
fn correct_rounds_to_count(shared_state: State<'_, SharedState>) {
    shared_state.lock().unwrap().data.correct_rounds_to_count();
}

#[tauri::command]
fn correct_bracket_to_count(shared_state: State<'_, SharedState>) {
    shared_state.lock().unwrap().data.correct_bracket_to_count();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Arc::new(Mutex::new(AppState::default())));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start_webserver,
            stop_webserver,
            get_loaded_config,
            get_settings,
            set_settings,
            get_division,
            set_division,
            get_resources,
            set_resources,
            get_current_match,
            set_current_match,
            load_from_filename,
            save_to_filename,
            to_relative_path,
            from_relative_path,
            correct_rounds_to_count,
            correct_bracket_to_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
