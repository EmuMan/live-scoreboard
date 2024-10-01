use std::fs;

use gtk::{self, gio, prelude::*};
use gtk::glib::clone;

use crate::{SharedState, SaveData};

pub fn open_config_file(
    parent_window: &gtk::ApplicationWindow,
    shared_state: SharedState,
) {
    let file_dialogue = gtk::FileDialog::builder()
        .title("Open Config File")
        .accept_label("Open")
        .modal(true)
        .filters(&get_json_filters())
        .build();

    let cancellable: Option<&gio::Cancellable> = None;
    file_dialogue.open(Some(parent_window), cancellable, clone!(
        #[strong] shared_state,
        move |result| {
            result.ok()
                .and_then(|file| file.path())
                .map(|path| {
                    read_into_state_from_config_file(shared_state.clone(), &path)
                });
        }
    ));
}

fn read_into_state_from_config_file(shared_state: SharedState, path: &std::path::PathBuf) {
    println!("Opening config file: {:?}", path);
    fs::read_to_string(path).ok()
        .map(|contents| {
            let new_data = serde_json::from_str::<SaveData>(&contents);
            match new_data {
                Ok(mut data) => {
                    data.correct_rounds_to_count();
                    let mut state = shared_state.lock().unwrap();
                    state.loaded_config = Some(path.clone());
                    state.data = data;
                    println!("Config file opened successfully: {:?}", path);
                }
                Err(err) => {
                    eprintln!("Error parsing config file: {:?}", err);
                }
            }
        });
}

pub fn save_config_file(
    parent_window: &gtk::ApplicationWindow,
    shared_state: crate::SharedState,
) {
    let file_dialogue = gtk::FileDialog::builder()
        .title("Save Config File")
        .accept_label("Save")
        .modal(true)
        .filters(&get_json_filters())
        .build();

    let cancellable: Option<&gio::Cancellable> = None;
    file_dialogue.save(Some(parent_window), cancellable, move |result| {
        result.ok()
            .and_then(|file| file.path())
            .map(|mut path| {
                path = path.with_extension("json");
                write_state_to_config_file(shared_state.clone(), &path)
            });
    });
}

fn write_state_to_config_file(shared_state: SharedState, path: &std::path::PathBuf) {
    println!("Saving config file: {:?}", path);
    let state = shared_state.lock().unwrap().clone();
    let serialized = serde_json::to_string(&state.data).unwrap();
    match fs::write(path, serialized) {
        Ok(_) => {
            let mut state = shared_state.lock().unwrap();
            state.loaded_config = Some(path.clone());
            println!("Config file saved successfully: {:?}", path);
        },
        Err(err) => eprintln!("Error saving config file: {:?}", err),
    }
}

fn get_json_filters() -> gio::ListStore {
    let filters = gio::ListStore::new::<gtk::FileFilter>();
    let filter = gtk::FileFilter::new();
    filter.add_pattern("*.json");
    filter.set_name(Some("JSON"));
    filters.append(&filter);
    filters
}

pub fn get_filters(filter_info: &Vec<(String, Vec<String>)>) -> gio::ListStore {
    let filters = gio::ListStore::new::<gtk::FileFilter>();
    for (name, patterns) in filter_info {
        let filter = gtk::FileFilter::new();
        for pattern in patterns {
            filter.add_pattern(&pattern);
        }
        filter.set_name(Some(&name));
        filters.append(&filter);
    }
    filters
}

pub fn to_web_path(path: &std::path::Path) -> Option<String> {
    let current_dir = std::env::current_dir().unwrap();
    path.strip_prefix(current_dir).ok()
        .map(|p| std::path::Path::new(".").join(p))
        .map(|p| p.to_string_lossy().to_string())
        .map(|p| p.replace("\\", "/"))
        .map(|p| p.trim_start_matches(".").to_string())
}

pub fn from_web_path(path: &str) -> String {
    let path = path.replace("\\", "/");
    if path.starts_with("/assets") {
        let current_dir = std::env::current_dir().unwrap();
        let path = path.trim_start_matches("/");
        let path = std::path::Path::new(path);
        current_dir.join(path).to_string_lossy().to_string()
    } else {
        path.into()
    }
}
