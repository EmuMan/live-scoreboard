use std::fs;
use crate::{AppState, models::SaveData};


pub fn read_into_state_from_config_file(state: &mut AppState, path: &std::path::Path) {
    println!("Opening config file: {:?}", path);
    fs::read_to_string(path).ok()
        .map(|contents| {
            let new_data = serde_json::from_str::<SaveData>(&contents);
            match new_data {
                Ok(mut data) => {
                    data.correct_rounds_to_count();
                    state.loaded_config = Some(path.to_path_buf());
                    state.data = data;
                    println!("Config file opened successfully: {:?}", path);
                }
                Err(err) => {
                    eprintln!("Error parsing config file: {:?}", err);
                }
            }
        });
}

pub fn write_state_to_config_file(state: &mut AppState, path: &std::path::Path) {
    println!("Saving config file: {:?}", path);
    let serialized = serde_json::to_string(&state.data).unwrap();
    match fs::write(path, serialized) {
        Ok(_) => {
            state.loaded_config = Some(path.to_path_buf());
            println!("Config file saved successfully: {:?}", path);
        },
        Err(err) => eprintln!("Error saving config file: {:?}", err),
    }
}

pub fn remove_file_from_path(path: &std::path::Path) -> std::path::PathBuf {
    let mut path = path.to_path_buf();
    path.pop();
    path
}

// prefixed with root! not dot!
pub fn to_relative_path(base_path: &std::path::Path, path: &std::path::Path) -> Option<String> {
    path.strip_prefix(base_path).ok()
        .map(|p| std::path::Path::new(".").join(p))
        .map(|p| p.to_string_lossy().to_string())
        .map(|p| p.replace("\\", "/"))
        .map(|p| p.trim_start_matches(".").to_string())
}

pub fn from_relative_path(base_path: &std::path::Path, path: &str) -> String {
    let path = path.trim_start_matches("/");
    let path = std::path::Path::new(path);
    base_path.join(path).to_string_lossy().to_string()
}
