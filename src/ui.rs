pub mod settings;
pub mod teams;

use std::sync::{Arc, Mutex};

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use crate::models::Division;
use crate::AppState;

pub fn build_ui(app: &Application) {
    let shared_state = Arc::new(Mutex::new(AppState {
        division: Division::new("Test Division", Vec::new())
    }));

    let notebook = gtk::Notebook::builder()
        .scrollable(true)
        .build();

    let teams_box = teams::build_box(shared_state.clone());
    let teams_label = gtk::Label::new(Some("Teams"));
    notebook.append_page(&teams_box, Some(&teams_label));

    let settings_box = settings::build_box(shared_state.clone());
    let settings_label = gtk::Label::new(Some("Settings"));
    notebook.append_page(&settings_box, Some(&settings_label));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Live Scoreboard")
        .default_width(500)
        .default_height(1000)
        .child(&notebook)
        .build();

    window.present();
}
