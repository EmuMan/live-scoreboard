pub mod pages;
pub mod components;

use std::collections::HashMap;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::glib::clone;
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
        .default_width(500)
        .default_height(1000)
        .build();

    let notebook = build_notebook(&window, shared_state.clone());

    // quick hack to tell pages when they've been switched to
    // don't wanna set up custom signals for this...
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

fn make_box(orientation: gtk::Orientation) -> gtk::Box {
    gtk::Box::builder()
        .orientation(orientation)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn clear_box(box_: &gtk::Box) {
    let mut first_child = box_.first_child();
    while let Some(child) = first_child {
        box_.remove(&child);
        first_child = box_.first_child();
    }
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

fn make_new_window(
    primary_window: &gtk::ApplicationWindow,
    title: &str,
    contents: &gtk::Box,
) -> gtk::Window {
    gtk::Window::builder()
        .transient_for(primary_window)
        .modal(true)
        .title(title)
        .default_width(200)
        .default_height(200)
        .child(contents)
        .build()
}

fn make_entry_window(
    primary_window: &gtk::ApplicationWindow,
    title: &str,
    field_names: Vec<String>,
    on_submit: Box<dyn Fn(HashMap<String, String>)>,
) {
    let entry_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let mut entries = HashMap::new();

    for field_name in &field_names {
        let entry = make_entry();
        entry.set_placeholder_text(Some(&field_name));
        entry_box.append(&entry);
        entries.insert(field_name.clone(), entry);
    }

    let submit_button = make_button("Submit");

    entry_box.append(&submit_button);

    let window = make_new_window(primary_window, title, &entry_box);
    
    submit_button.connect_clicked(clone!(
        #[weak] window,
        #[strong] entries,
        move |_| {
            let results = entries.iter()
                .map(|(field_name, field_info)| {
                    (field_name.to_string(), field_info.text().to_string())
                })
                .collect();
            on_submit(results);
            window.close();
        }
    ));

    window.present();
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

fn get_model_with_none(options: &Vec<String>) -> gtk::StringList{
    let mut with_none = vec!["(none)"];
    let mut options: Vec<&str> = options.iter().map(|team| team.as_str()).collect();
    with_none.append(&mut options);
    gtk::StringList::new(&with_none)
}

fn index_of_or_none(list: &Vec<String>, item: &Option<String>) -> Option<usize> {
    match item {
        Some(item) => list.iter().position(|x| x == item),
        None => None,
    }
}
