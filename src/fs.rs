use std::fs;

use gtk::{self, gio, prelude::FileExt};
use gtk::glib::clone;

pub fn open_config_file(
    parent_window: &gtk::ApplicationWindow,
    shared_state: crate::SharedState,
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
                .and_then(|path| {
                    println!("Opening config file: {:?}", path);
                    fs::read_to_string(path).ok()
                })
                .map(|contents| {
                    let new_data = serde_json::from_str::<crate::AppState>(&contents);
                    match new_data {
                        Ok(mut data) => {
                            data.correct_rounds_to_count();
                            let mut state = shared_state.lock().unwrap();
                            *state = data;
                        }
                        Err(err) => {
                            eprintln!("Error parsing config file: {:?}", err);
                        }
                    }
                });
        }
    ));
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
            .and_then(|mut path| {
                path = path.with_extension("json");
                println!("Saving config file: {:?}", path);
                let contents = shared_state.lock().unwrap().clone();
                let serialized = serde_json::to_string(&contents).unwrap();
                fs::write(path, serialized).ok()
            });
    });
}

fn get_json_filters() -> gio::ListStore {
    let filters = gio::ListStore::new::<gtk::FileFilter>();
    let filter = gtk::FileFilter::new();
    filter.add_pattern("*.json");
    filter.set_name(Some("JSON"));
    filters.append(&filter);
    filters
}
