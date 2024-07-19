use gtk::prelude::*;
use gtk::{Label, Box, ScrolledWindow, ListBox};
use gtk::glib::clone;
use gtk::glib;

pub fn build_box() -> Box {
    let gtk_box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let (list_box, teams_list) = build_teams_list();

    let team_info_container = Box::builder().build();

    let team_info = build_team_info("None");
    team_info_container.append(&team_info);
    
    list_box.connect_row_selected(clone!(
        #[weak]
        team_info_container,
        move |_, row| {
            if let Some(row) = row {
                let team = row.child().unwrap().downcast::<Label>().unwrap().label();
                let current_info = team_info_container.first_child();
                if let Some(current_info) = current_info {
                    team_info_container.remove(&current_info);
                }
                team_info_container.append(&build_team_info(&team));
            }
        }
    ));

    gtk_box.append(&teams_list);
    gtk_box.append(&team_info_container);

    gtk_box
}

fn build_teams_list() -> (ListBox, ScrolledWindow) {
    let list_box = ListBox::new();
    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_width(360)
        .min_content_height(360)
        .child(&list_box)
        .build();

    (list_box, scrolled_window)
}

fn build_team_info(team_name: &str) -> Box {
    let gtk_box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let label = Label::builder()
        .label(format!("Team: {}", team_name).as_str())
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    gtk_box.append(&label);

    gtk_box
}
