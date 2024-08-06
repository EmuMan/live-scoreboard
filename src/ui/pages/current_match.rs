use gtk::prelude::*;
use gtk::glib::closure_local;
use gtk::glib::{self, clone};

use crate::{models, ui::{util, components::refresh_box::RefreshBox}, SharedState, AppState};

pub fn build_box(_window: &gtk::ApplicationWindow, shared_state: SharedState) -> RefreshBox {

    //////////////////
    // DECLARATIONS //
    //////////////////
    
    let refresh_box = RefreshBox::new();
    refresh_box.set_orientation(gtk::Orientation::Vertical);

    let teams_container = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    let map_progress_container = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);

    /////////////////
    // CONNECTIONS //
    /////////////////

    refresh_box.connect_closure(
        "refresh-status",
        false,
        closure_local!(
            #[strong] shared_state,
            #[weak] teams_container,
            #[weak] map_progress_container,
            move |_refresh_box: RefreshBox, new_status: bool| {
                if new_status {
                    shared_state.lock().unwrap().correct_rounds_to_count();
                    set_teams(&teams_container, shared_state.clone());
                    set_map_progress_box(shared_state.clone(), map_progress_container)
                } else {
                    util::clear_box(&teams_container);
                    util::clear_box(&map_progress_container);
                }
            }
        )
    );

    /////////////////
    // ARRANGEMENT //
    /////////////////

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

    let team_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
    let team_label = gtk::Label::new(Some(format!("Team {}", number).as_str()));
    let team_logo_container = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    
    let name_dropdown_model = util::get_model_with_none(&state.team_names());
    let dropdown = gtk::DropDown::new(Some(name_dropdown_model), gtk::Expression::NONE);
    let team_index = if number == 1 { state.current_match.team1 } else { state.current_match.team2 };
    if let Some(team_index) = team_index {
        dropdown.set_selected(team_index as u32 + 1);
        let team_logo_path = &state.division.teams.get(team_index).unwrap().icon;
        team_logo_container.append(&get_team_icon(team_logo_path.as_deref()));
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
                let team = &state.division.teams[team_index];
                team_logo_container.append(&get_team_icon(team.icon.as_deref()));
            }
        }
    ));

    team_box.append(&team_label);
    team_box.append(&dropdown);
    team_box.append(&team_logo_container);
    
    team_box
}

fn get_team_icon(path: Option<&str>) -> gtk::Image {
    if let Some(path) = path {
        util::load_image(path, 200, 200)
    } else {
        gtk::Image::from_icon_name("image-missing")
    }
}

fn set_map_progress_box(shared_state: SharedState, map_progress_box: gtk::Box) {
    let state = shared_state.lock().unwrap();
    for (i, round) in state.current_match.rounds.iter().enumerate() {
        map_progress_box.append(&make_map_box(shared_state.clone(), &state, round, i));
    }
}

fn make_map_box(shared_state: SharedState, state: &AppState, round: &models::Round, round_index: usize) -> gtk::Box {
    let map_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);

    let map_info_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);

    let gamemode_label = gtk::Label::new(Some("Gamemode"));
    let gamemode_names = state.settings.gamemodes.iter().map(|g| g.name.clone()).collect();
    let gamemode_model = util::get_model_with_none(&gamemode_names);
    let gamemode_dropdown = gtk::DropDown::new(Some(gamemode_model), gtk::Expression::NONE);
    let gamemode_index = util::index_of_or_none(&gamemode_names, &round.gamemode);
    let gamemode = gamemode_index.and_then(|index| state.settings.gamemodes.get(index));
    if let Some(index) = gamemode_index {
        gamemode_dropdown.set_selected(index as u32 + 1);
    } else {
        gamemode_dropdown.set_selected(0);
        // TODO: set gamemode in app state to none
    }

    let map_label = gtk::Label::new(Some("Map"));
    let map_names = gamemode
        .and_then(|gamemode| Some(gamemode.maps.iter().map(|m| m.name.clone()).collect()))
        .unwrap_or(Vec::new());
    let map_model = util::get_model_with_none(&map_names);
    let map_dropdown = gtk::DropDown::new(Some(map_model), gtk::Expression::NONE);
    if let Some(index) = util::index_of_or_none(&map_names, &round.map) {
        map_dropdown.set_selected(index as u32 + 1);
    } else {
        map_dropdown.set_selected(0);
        // TODO: set map in app state to none
    }

    gamemode_dropdown.connect_selected_notify(clone!(
        #[strong] shared_state,
        #[weak] map_dropdown,
        move |dropdown| {
            let selected_index = dropdown.selected();
            let new_map_model;
            {
                let mut state = shared_state.lock().unwrap();
                let gamemode = if selected_index == 0 {
                    new_map_model = util::get_model_with_none(&Vec::new());
                    None
                } else {
                    let gamemode = state.settings.gamemodes.get((selected_index - 1) as usize)
                        .and_then(|gamemode| Some(gamemode.clone()));
                    let map_names: Option<Vec<String>> = gamemode
                        .as_ref()
                        .and_then(|gamemode| Some(gamemode.maps.iter().map(|m| m.name.clone()).collect()));
                    new_map_model = util::get_model_with_none(&map_names.unwrap_or(Vec::new()));
                    gamemode
                };
                if let Some(round) = state.current_match.rounds.get_mut(round_index) {
                    round.gamemode = gamemode.and_then(|gamemode| Some(gamemode.name));
                }
            }
            map_dropdown.set_model(Some(&new_map_model));
            map_dropdown.set_selected(0);
        }
    ));

    map_dropdown.connect_selected_notify(clone!(
        #[strong] shared_state,
        #[weak] gamemode_dropdown,
        move |dropdown| {
            let selected_index = dropdown.selected();
            let mut state = shared_state.lock().unwrap();

            let gamemode_index = gamemode_dropdown.selected();
            if gamemode_index == 0 {
                return;
            }
            if let Some(gamemode) = state.settings.gamemodes.get((gamemode_index - 1) as usize) {
                let selected_map = if selected_index == 0 {
                    None
                } else {
                    Some(gamemode.maps[(selected_index - 1) as usize].clone())
                };
                if let Some(round) = state.current_match.rounds.get_mut(round_index) {
                    round.map = selected_map.and_then(|map| Some(map.name));
                }
            }
        }
    ));

    map_info_box.append(&gamemode_label);
    map_info_box.append(&gamemode_dropdown);
    map_info_box.append(&map_label);
    map_info_box.append(&map_dropdown);

    let scores_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);

    let team1_score_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
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

    let team2_score_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
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

    let completed_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
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
