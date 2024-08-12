use gtk::prelude::*;

pub fn make_button(label: &str, mtop: usize, mbot: usize, msta: usize, mend: usize) -> gtk::Button {
    gtk::Button::builder()
        .label(label)
        .margin_top(mtop as i32)
        .margin_bottom(mbot as i32)
        .margin_start(msta as i32)
        .margin_end(mend as i32)
        .build()
}

pub fn make_label(label: &str, mtop: usize, mbot: usize, msta: usize, mend: usize) -> gtk::Label {
    gtk::Label::builder()
        .label(label)
        .margin_top(mtop as i32)
        .margin_bottom(mbot as i32)
        .margin_start(msta as i32)
        .margin_end(mend as i32)
        .build()
}

pub fn make_entry(mtop: usize, mbot: usize, msta: usize, mend: usize) -> gtk::Entry {
    gtk::Entry::builder()
        .margin_top(mtop as i32)
        .margin_bottom(mbot as i32)
        .margin_start(msta as i32)
        .margin_end(mend as i32)
        .build()
}

pub fn make_spin_button(mtop: usize, mbot: usize, msta: usize, mend: usize) -> gtk::SpinButton {
    gtk::SpinButton::builder()
        .adjustment(&gtk::Adjustment::new(0.0, 0.0, 100.0, 1.0, 1.0, 1.0))
        .digits(0)
        .numeric(true)
        .margin_top(mtop as i32)
        .margin_bottom(mbot as i32)
        .margin_start(msta as i32)
        .margin_end(mend as i32)
        .build()
}

pub fn make_box(orientation: gtk::Orientation, mtop: usize, mbot: usize, msta: usize, mend: usize) -> gtk::Box {
    gtk::Box::builder()
        .orientation(orientation)
        .margin_top(mtop as i32)
        .margin_bottom(mbot as i32)
        .margin_start(msta as i32)
        .margin_end(mend as i32)
        .build()
}

pub fn make_frame(label: &str, mtop: usize, mbot: usize, msta: usize, mend: usize) -> gtk::Frame {
    gtk::Frame::builder()
        .label(label)
        .margin_top(mtop as i32)
        .margin_bottom(mbot as i32)
        .margin_start(msta as i32)
        .margin_end(mend as i32)
        .build()
}

pub fn clear_box(box_: &gtk::Box) {
    let mut first_child = box_.first_child();
    while let Some(child) = first_child {
        box_.remove(&child);
        first_child = box_.first_child();
    }
}

pub fn make_list(mtop: usize, mbot: usize, msta: usize, mend: usize) -> (gtk::ListBox, gtk::ScrolledWindow) {
    let list_box = gtk::ListBox::new();

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_width(360)
        .min_content_height(240)
        .margin_top(mtop as i32)
        .margin_bottom(mbot as i32)
        .margin_start(msta as i32)
        .margin_end(mend as i32)
        .child(&list_box)
        .build();

    (list_box, scrolled_window)
}

pub fn make_new_window(
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

pub fn load_image(path: &str, width: usize, height: usize) -> gtk::Image {
    let image = gtk::Image::builder()
        .file(path)
        .build();
    image.set_size_request(width as i32, height as i32);
    image
}

pub fn get_model_with_none(options: &Vec<String>) -> gtk::StringList{
    let mut with_none = vec!["(none)"];
    let mut options: Vec<&str> = options.iter().map(|team| team.as_str()).collect();
    with_none.append(&mut options);
    gtk::StringList::new(&with_none)
}

pub fn index_of_or_none(list: &Vec<String>, item: &Option<String>) -> Option<usize> {
    match item {
        Some(item) => list.iter().position(|x| x == item),
        None => None,
    }
}