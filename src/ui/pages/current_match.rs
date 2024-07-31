use gtk::prelude::*;
use gtk::glib::closure_local;
use gtk::glib::{self, clone};

use crate::{ui::components::refresh_box, SharedState};

pub fn build_box(_window: &gtk::ApplicationWindow, shared_state: SharedState) -> refresh_box::RefreshBox {
    let refresh_box = refresh_box::RefreshBox::new();
    refresh_box.set_orientation(gtk::Orientation::Vertical);

    let teams_container = crate::ui::make_box(gtk::Orientation::Horizontal);
    let map_progress_container = crate::ui::make_box(gtk::Orientation::Horizontal);

    refresh_box.connect_closure(
        "refresh-status",
        false,
        closure_local!(
            #[strong] shared_state,
            #[weak] teams_container,
            move |_refresh_box: refresh_box::RefreshBox, new_status: bool| {
                if new_status {
                    set_teams(&teams_container, shared_state.clone());
                } else {
                    let mut first_child = teams_container.first_child();
                    while let Some(child) = first_child {
                        teams_container.remove(&child);
                        first_child = teams_container.first_child();
                    }
                }
            }
        )
    );

    refresh_box.append(&teams_container);
    refresh_box.append(&map_progress_container);

    refresh_box
}

fn set_teams(teams_container: &gtk::Box, shared_state: SharedState) {
    let team_1_box = build_team_box(1, shared_state.clone());
    let team_2_box = build_team_box(2, shared_state);

    teams_container.append(&team_1_box);
    teams_container.append(&team_2_box);
}

fn build_team_box(number: usize, shared_state: SharedState) -> gtk::Box {
    let state = shared_state.lock().unwrap();

    let team_box = crate::ui::make_box(gtk::Orientation::Vertical);
    let team_label = gtk::Label::new(Some(format!("Team {}", number).as_str()));
    let team_logo_container = crate::ui::make_box(gtk::Orientation::Horizontal);
    
    let name_dropdown_model = state.team_names_model();
    let dropdown = gtk::DropDown::new(Some(name_dropdown_model), gtk::Expression::NONE);
    let team_index = if number == 1 { state.current_match.team1 } else { state.current_match.team2 };
    if let Some(team_index) = team_index {
        dropdown.set_selected(team_index as u32 + 1);
        let team_logo = get_team_logo(&state.division.teams.get(team_index).unwrap().name);
        team_logo_container.append(&team_logo);
    } else {
        dropdown.set_selected(0);
    }

    dropdown.connect_selected_notify(
        clone!(
        #[strong] shared_state,
        #[weak] team_logo_container,
        move |dropdown| {
            let selected_index = dropdown.selected();
            let mut state = shared_state.lock().unwrap();
            let team_index = if number == 1 { &mut state.current_match.team1 } else { &mut state.current_match.team2 };
            *team_index = if selected_index == 0 {
                None
            } else {
                Some((selected_index - 1) as usize)
            };
            
            let first_child = team_logo_container.first_child();
            if let Some(first_child) = first_child {
                team_logo_container.remove(&first_child);
            }
            if let Some(team_index) = *team_index {
                let team_name = &state.team_names()[team_index];
                let team_logo = get_team_logo(team_name);
                team_logo_container.append(&team_logo);
            }
        }
    ));

    team_box.append(&team_label);
    team_box.append(&dropdown);
    team_box.append(&team_logo_container);
    
    team_box
}

fn get_team_logo(team_name: &str) -> gtk::Label {
    let team_logo = gtk::Label::new(Some(team_name));
    team_logo
}
