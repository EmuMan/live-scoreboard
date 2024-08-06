pub mod pages;
pub mod components;

use std::collections::HashMap;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::glib::clone;
use gtk::glib;
use gtk::gio;

use crate::models::{Division, Match, Settings};
use crate::{fs, AppState, SharedState};
use components::refresh_box;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum EntryWindowField {
    Text { label: String, prefill: Option<String> },
    DropDown { label: String, options: Vec<String>, prefill: Option<String> },
    File { label: String, filters: Vec<(String, Vec<String>)>, prefill: Option<String> },
}

impl EntryWindowField {
    pub fn label(&self) -> &str {
        match self {
            EntryWindowField::Text { label, .. } => label,
            EntryWindowField::DropDown { label, .. } => label,
            EntryWindowField::File { label, .. } => label,
        }
    }
}

pub trait HasResult {
    fn result(&self) -> Option<String>;
}

impl HasResult for gtk::Entry {
    fn result(&self) -> Option<String> {
        let text = self.text().to_string();
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}

impl HasResult for gtk::DropDown {
    fn result(&self) -> Option<String> {
        let selected = self.selected();
        if selected == 0 {
            None
        } else {
            Some(self.model()?.item(selected as u32)?.downcast::<gtk::StringObject>().unwrap().string().to_string())
        }
    }
}

impl HasResult for gtk::Label {
    fn result(&self) -> Option<String> {
        let text = self.label().to_string();
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}

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

fn open_entry_window(
    primary_window: &gtk::ApplicationWindow,
    title: &str,
    fields: Vec<EntryWindowField>,
    on_submit: Box<dyn Fn(HashMap<String, Option<String>>)>,
) {
    let entry_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let window = make_new_window(primary_window, title, &entry_box);

    // map of field to widget
    let mut field_widgets: HashMap<EntryWindowField, Box<dyn HasResult>> = HashMap::new();

    for field in &fields {
        match field {
            EntryWindowField::Text { label, prefill } => {
                let entry = make_entry();
                entry.set_placeholder_text(Some(label));
                if let Some(prefill) = prefill {
                    entry.set_text(prefill);
                }
                entry_box.append(&entry);
                field_widgets.insert(field.clone(), Box::new(entry));
            },
            EntryWindowField::DropDown { label, options, prefill } => {
                let field_box = make_box(gtk::Orientation::Horizontal);
                let label = make_label(label);
                let model = get_model_with_none(options);
                let dropdown = gtk::DropDown::builder()
                    .model(&model)
                    .build();
                prefill.as_ref()
                    .and_then(|prefill| index_of_or_none(options, &Some(prefill.to_string()))
                    .map(|index| dropdown.set_selected((index + 1) as u32)));
                field_box.append(&label);
                field_box.append(&dropdown);
                entry_box.append(&field_box);
                field_widgets.insert(field.clone(), Box::new(dropdown));
            },
            EntryWindowField::File { label, filters, prefill } => {
                let file_box = make_box(gtk::Orientation::Vertical);
                let file_button = make_button(label);
                let file_label = make_label(prefill.as_ref().unwrap_or(&String::from("(none)")));

                let file_dialog = gtk::FileDialog::builder()
                    .title(label)
                    .accept_label("Select")
                    .modal(true)
                    .filters(&fs::get_filters(filters))
                    .build();

                file_box.append(&file_button);
                file_box.append(&file_label);
                entry_box.append(&file_box);
                
                file_button.connect_clicked(clone!(
                    #[strong] file_dialog,
                    #[weak] window,
                    #[weak] file_label,
                    move |_| {
                        let cancellable: Option<&gio::Cancellable> = None;
                        file_dialog.open(Some(&window), cancellable, clone!(
                            #[weak] file_label,
                            move |result| {
                                result.ok()
                                    .and_then(|file| file.path())
                                    .and_then(|path| Some(path.to_string_lossy().to_string()))
                                    .map(|path| file_label.set_label(path.as_str()));
                            }
                        ));
                    }
                ));

                field_widgets.insert(field.clone(), Box::new(file_label));
            },
        }
    }

    let submit_button = make_button("Submit");

    entry_box.append(&submit_button);
    
    submit_button.connect_clicked(clone!(
        #[weak] window,
        move |_| {
            let results = field_widgets.iter()
                .map(|(field_info, widget)| {
                    (field_info.label().to_string(), widget.result())
                })
                .collect();
            on_submit(results);
            window.close();
        }
    ));

    window.present();
}

pub fn load_image(path: &str, width: usize, height: usize) -> gtk::Image {
    let image = gtk::Image::builder()
        .file(path)
        .build();
    image.set_size_request(width as i32, height as i32);
    image
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
