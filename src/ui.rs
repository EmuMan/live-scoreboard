pub mod pages;
pub mod components;
pub mod synced_list_box;
pub mod entry_window;
pub mod util;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::glib;

use crate::models::{Division, Match, Settings};
use crate::{AppState, SharedState};
use components::refresh_box;

pub fn build_ui(app: &Application) {
    let shared_state = AppState::new_shared(
        Settings::default(),
        Division::default(),
        Vec::new(),
        Match::default(),
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Live Scoreboard")
        .default_width(1500)
        .default_height(1000)
        .build();

    window.settings().set_gtk_theme_name(Some("Default-hc-dark"));
    window.settings().set_gtk_font_name(Some("Segoe UI 12"));

    let notebook = build_notebook(&window, shared_state.clone());

    notebook.connect_switch_page(move |notebook, page, page_num| {
        for (i, page) in notebook.pages().iter::<glib::Object>().enumerate() {
            if i == page_num as usize {
                continue;
            }
            if let Ok(page) = page {
                let page = page.downcast::<gtk::NotebookPage>().unwrap();
                let refresh_box = page.child().downcast::<refresh_box::RefreshBox>().unwrap();
                refresh_box.emit_refresh_status(false);
            }
        }
        
        let refresh_box = page.clone().downcast::<refresh_box::RefreshBox>().unwrap();
        refresh_box.emit_refresh_status(true);
    });

    window.set_child(Some(&notebook));

    window.present();
}

pub fn build_notebook(window: &ApplicationWindow, shared_state: SharedState) -> gtk::Notebook {
    let notebook = gtk::Notebook::builder()
        .scrollable(true)
        .build();

    let teams_box = pages::teams::build_box(window, shared_state.clone());
    let teams_label = gtk::Label::new(Some("Teams"));
    notebook.append_page(&teams_box, Some(&teams_label));

    let bracket_box = pages::bracket::build_box(window, shared_state.clone());
    let bracket_label = gtk::Label::new(Some("Bracket"));
    notebook.append_page(&bracket_box, Some(&bracket_label));

    let current_match_box = pages::current_match::build_box(window, shared_state.clone());
    let current_match_label = gtk::Label::new(Some("Current Match"));
    notebook.append_page(&current_match_box, Some(&current_match_label));

    let assets_box = pages::assets::build_box(window, shared_state.clone());
    let assets_label = gtk::Label::new(Some("Assets"));
    notebook.append_page(&assets_box, Some(&assets_label));

    let settings_box = pages::settings::build_box(window, shared_state.clone());
    let settings_label = gtk::Label::new(Some("Settings"));
    notebook.append_page(&settings_box, Some(&settings_label));

    notebook
}
