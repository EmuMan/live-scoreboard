use std::collections::HashMap;

use gtk::prelude::*;
use gtk::glib::clone;
use gtk::glib;
use gtk::gio;

use crate::ui::util;

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

pub fn open_entry_window(
    primary_window: &gtk::ApplicationWindow,
    title: &str,
    fields: Vec<EntryWindowField>,
    on_submit: Box<dyn Fn(HashMap<String, Option<String>>)>,
) {
    let entry_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let window = util::make_new_window(primary_window, title, &entry_box);

    // map of field to widget
    let mut field_widgets: HashMap<EntryWindowField, Box<dyn HasResult>> = HashMap::new();

    for field in &fields {
        match field {
            EntryWindowField::Text { label, prefill } => {
                let entry = util::make_entry(12, 12, 12, 12);
                entry.set_placeholder_text(Some(label));
                if let Some(prefill) = prefill {
                    entry.set_text(prefill);
                }
                entry_box.append(&entry);
                field_widgets.insert(field.clone(), Box::new(entry));
            },
            EntryWindowField::DropDown { label, options, prefill } => {
                let field_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
                let label = util::make_label(label, 12, 12, 12, 12);
                let model = util::get_model_with_none(options);
                let dropdown = gtk::DropDown::builder()
                    .model(&model)
                    .build();
                prefill.as_ref()
                    .and_then(|prefill| util::index_of_or_none(options, &Some(prefill.to_string()))
                    .map(|index| dropdown.set_selected((index + 1) as u32)));
                field_box.append(&label);
                field_box.append(&dropdown);
                entry_box.append(&field_box);
                field_widgets.insert(field.clone(), Box::new(dropdown));
            },
            EntryWindowField::File { label, filters, prefill } => {
                let file_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
                let file_button = util::make_button(label, 12, 12, 12, 12);
                let file_label = util::make_label(
                    prefill.as_ref().unwrap_or(&String::from("(none)")),
                    12, 12, 12, 12
                );

                let file_dialog = gtk::FileDialog::builder()
                    .title(label)
                    .accept_label("Select")
                    .modal(true)
                    .filters(&crate::fs::get_filters(filters))
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
                                    .and_then(|path| crate::fs::to_web_path(&path))
                                    .map(|path| file_label.set_label(path.as_str()));
                            }
                        ));
                    }
                ));

                field_widgets.insert(field.clone(), Box::new(file_label));
            },
        }
    }

    let submit_button = util::make_button("Submit", 12, 12, 12, 12);

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
