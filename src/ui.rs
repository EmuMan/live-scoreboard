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

fn make_button(label: &str) -> gtk::Button {
    gtk::Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn make_label(label: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn make_entry() -> gtk::Entry {
    gtk::Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn make_list() -> (gtk::ListBox, gtk::ScrolledWindow) {
    let list_box = gtk::ListBox::new();

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_width(360)
        .min_content_height(240)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .child(&list_box)
        .build();

    (list_box, scrolled_window)
}

fn get_string_from_label_row(row: &gtk::ListBoxRow) -> Option<String> {
    let label = row.child()?.downcast::<gtk::Label>();
    match label {
        Ok(label) => Some(label.label().to_string()),
        Err(_) => None,
    }
}

fn get_string_from_box_row(row: &gtk::ListBoxRow) -> Option<String> {
    let box_ = row.child()?.downcast::<gtk::Box>();
    match box_ {
        Ok(box_) => {
            let label = box_.first_child()?.downcast::<gtk::Label>().ok()?;
            Some(label.label().to_string())
        },
        Err(_) => None,
    }
}
