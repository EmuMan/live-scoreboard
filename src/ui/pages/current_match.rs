use gtk::prelude::*;
use gtk::glib::closure_local;
use gtk::glib::{self, clone};

use crate::{models, ui::components::refresh_box, SharedState, AppState};

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
            #[weak] map_progress_container,
            move |_refresh_box: refresh_box::RefreshBox, new_status: bool| {
                if new_status {
                    shared_state.lock().unwrap().correct_rounds_to_count();
                    set_teams(&teams_container, shared_state.clone());
                    set_map_progress_box(shared_state.clone(), map_progress_container)
                } else {
                    crate::ui::clear_box(&teams_container);
                    crate::ui::clear_box(&map_progress_container);
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
    
    let name_dropdown_model = crate::ui::get_model_with_none(&state.team_names());
    let dropdown = gtk::DropDown::new(Some(name_dropdown_model), gtk::Expression::NONE);
    let team_index = if number == 1 { state.current_match.team1 } else { state.current_match.team2 };
    if let Some(team_index) = team_index {
        dropdown.set_selected(team_index as u32 + 1);
        let team_logo = get_team_logo(&state.division.teams.get(team_index).unwrap().name);
        team_logo_container.append(&team_logo);
    } else {
        dropdown.set_selected(0);
    }

    dropdown.connect_selected_notify(clone!(
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

fn set_map_progress_box(shared_state: SharedState, map_progress_box: gtk::Box) {
    let state = shared_state.lock().unwrap();
    for (i, round) in state.current_match.rounds.iter().enumerate() {
        map_progress_box.append(&make_map_box(shared_state.clone(), &state, round, i));
    }
}

fn make_map_box(shared_state: SharedState, state: &AppState, round: &models::Round, round_index: usize) -> gtk::Box {
    let map_box = crate::ui::make_box(gtk::Orientation::Vertical);

    let map_info_box = crate::ui::make_box(gtk::Orientation::Vertical);

    let gamemode_label = gtk::Label::new(Some("Gamemode"));
    let gamemode_model = crate::ui::get_model_with_none(&state.settings.gamemodes);
    let gamemode_dropdown = gtk::DropDown::new(Some(gamemode_model), gtk::Expression::NONE);
    if let Some(index) = crate::ui::index_of_or_none(&state.settings.gamemodes, &round.gamemode) {
        gamemode_dropdown.set_selected(index as u32 + 1);
    } else {
        gamemode_dropdown.set_selected(0);
        // TODO: set gamemode in app state to none
    }
    gamemode_dropdown.connect_selected_notify(clone!(
        #[strong] shared_state,
        move |dropdown| {
            let selected_index = dropdown.selected();
            let mut state = shared_state.lock().unwrap();
            let gamemode = if selected_index == 0 {
                None
            } else {
                Some(state.settings.gamemodes[(selected_index - 1) as usize].clone())
            };
            if let Some(round) = state.current_match.rounds.get_mut(round_index) {
                round.gamemode = gamemode;
            }
        }
    ));

    let map_name_label = gtk::Label::new(Some("Map"));
    let map_name_model = crate::ui::get_model_with_none(&state.settings.maps);
    let map_name_dropdown = gtk::DropDown::new(Some(map_name_model), gtk::Expression::NONE);
    if let Some(index) = crate::ui::index_of_or_none(&state.settings.maps, &round.map) {
        map_name_dropdown.set_selected(index as u32 + 1);
    } else {
        map_name_dropdown.set_selected(0);
        // TODO: set map in app state to none
    }
    map_name_dropdown.connect_selected_notify(clone!(
        #[strong] shared_state,
        move |dropdown| {
            let selected_index = dropdown.selected();
            let mut state = shared_state.lock().unwrap();
            let map = if selected_index == 0 {
                None
            } else {
                Some(state.settings.maps[(selected_index - 1) as usize].clone())
            };
            if let Some(round) = state.current_match.rounds.get_mut(round_index) {
                round.map = map;
            }
        }
    ));

    map_info_box.append(&gamemode_label);
    map_info_box.append(&gamemode_dropdown);
    map_info_box.append(&map_name_label);
    map_info_box.append(&map_name_dropdown);

    let scores_box = crate::ui::make_box(gtk::Orientation::Vertical);

    let team1_score_box = crate::ui::make_box(gtk::Orientation::Vertical);
    let team1_score_label = gtk::Label::new(Some("Team 1 Score"));
    let team1_score_entry = gtk::SpinButton::builder()
        .adjustment(&gtk::Adjustment::new(0.0, 0.0, 100.0, 1.0, 1.0, 1.0))
        .digits(0)
        .numeric(true)
        .build();
    team1_score_entry.set_value(round.team1_score as f64);
    team1_score_entry.connect_changed(clone!(
        #[strong] shared_state,
        move |entry| {
            let score = entry.value() as u32;
            let mut state = shared_state.lock().unwrap();
            if let Some(round) = state.current_match.rounds.get_mut(round_index) {
                round.team1_score = score as usize;
            }
        }
    ));

    let team2_score_box = crate::ui::make_box(gtk::Orientation::Vertical);
    let team2_score_label = gtk::Label::new(Some("Team 2 Score"));
    let team2_score_entry = gtk::SpinButton::builder()
        .adjustment(&gtk::Adjustment::new(0.0, 0.0, 100.0, 1.0, 1.0, 1.0))
        .digits(0)
        .numeric(true)
        .build();
    team2_score_entry.set_value(round.team2_score as f64);
    team2_score_entry.connect_changed(clone!(
        #[strong] shared_state,
        move |entry| {
            let score = entry.value() as u32;
            let mut state = shared_state.lock().unwrap();
            if let Some(round) = state.current_match.rounds.get_mut(round_index) {
                round.team2_score = score as usize;
            }
        }
    ));

    let completed_box = crate::ui::make_box(gtk::Orientation::Horizontal);
    let completed_label = gtk::Label::new(Some("Completed "));
    let completed_switch = gtk::Switch::builder().active(round.completed).build();
    completed_switch.connect_state_set(clone!(
        #[strong] shared_state,
        move |_, switch_state| {
            let mut state = shared_state.lock().unwrap();
            if let Some(round) = state.current_match.rounds.get_mut(round_index) {
                round.completed = switch_state;
            }
            glib::signal::Propagation::Proceed
        }
    ));

    team1_score_box.append(&team1_score_label);
    team1_score_box.append(&team1_score_entry);
    team2_score_box.append(&team2_score_label);
    team2_score_box.append(&team2_score_entry);

    scores_box.append(&team1_score_box);
    scores_box.append(&team2_score_box);

    completed_box.append(&completed_label);
    completed_box.append(&completed_switch);

    map_box.append(&map_info_box);
    map_box.append(&scores_box);
    map_box.append(&completed_box);
    
    map_box
}
