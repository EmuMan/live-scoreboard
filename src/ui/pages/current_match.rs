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
    let round_progress_container = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);

    /////////////////
    // CONNECTIONS //
    /////////////////

    refresh_box.connect_closure(
        "refresh-status",
        false,
        closure_local!(
            #[strong] shared_state,
            #[weak] teams_container,
            #[weak] round_progress_container,
            move |_refresh_box: RefreshBox, new_status: bool| {
                if new_status {
                    shared_state.lock().unwrap().correct_rounds_to_count();
                    set_teams(&teams_container, shared_state.clone());
                    set_round_progress_box(shared_state.clone(), round_progress_container)
                } else {
                    util::clear_box(&teams_container);
                    util::clear_box(&round_progress_container);
                }
            }
        )
    );

    /////////////////
    // ARRANGEMENT //
    /////////////////

    refresh_box.append(&teams_container);
    refresh_box.append(&round_progress_container);

    refresh_box
}

fn set_teams(teams_container: &gtk::Box, shared_state: SharedState) {

    //////////////////
    // DECLARATIONS //
    //////////////////

    let team_1_box = build_team_frame(1, shared_state.clone());
    let team_2_box = build_team_frame(2, shared_state.clone());

    let swap_scoreboard_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
    let swap_scoreboard_label = util::make_label("Swap Scoreboard", 12, 12, 12, 12);
    let swap_scoreboard_switch = gtk::Switch::builder().build();

    ////////////////////
    // INITIALIZATION //
    ////////////////////

    {
        let state = shared_state.lock().unwrap();
        swap_scoreboard_switch.set_state(state.current_match.swap_scoreboard);
    }

    /////////////////
    // CONNECTIONS //
    /////////////////

    swap_scoreboard_switch.connect_state_set(clone!(
        #[strong] shared_state,
        move |_, switch_state| {
            let mut state = shared_state.lock().unwrap();
            state.current_match.swap_scoreboard = switch_state;
            glib::signal::Propagation::Proceed
        }
    ));

    /////////////////
    // ARRANGEMENT //
    /////////////////

    swap_scoreboard_box.append(&swap_scoreboard_label);
    swap_scoreboard_box.append(&swap_scoreboard_switch);

    teams_container.append(&team_1_box);
    teams_container.append(&swap_scoreboard_box);
    teams_container.append(&team_2_box);
}

fn build_team_frame(number: usize, shared_state: SharedState) -> gtk::Frame {

    //////////////////
    // DECLARATIONS //
    //////////////////

    let team_names = shared_state.lock().unwrap().team_names();
    let team_names_model = util::get_model_with_none(&team_names);

    let team_frame = util::make_frame(format!("Team {}", number).as_str(), 12, 12, 12, 12);
    let team_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
    let team_dropdown = gtk::DropDown::new(Some(team_names_model), gtk::Expression::NONE);
    let team_logo_container = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);

    ////////////////////
    // INITIALIZATION //
    ////////////////////

    let state = shared_state.lock().unwrap();

    let team_index = if number == 1 { state.current_match.team1 } else { state.current_match.team2 };
    if let Some(team_index) = team_index {
        team_dropdown.set_selected(team_index as u32 + 1);
        let team_logo_path = &state.division.teams.get(team_index).unwrap().icon;
        team_logo_container.append(&get_team_icon(team_logo_path.as_deref()));
    } else {
        team_dropdown.set_selected(0);
    }

    /////////////////
    // CONNECTIONS //
    /////////////////

    team_dropdown.connect_selected_notify(clone!(
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

    /////////////////
    // ARRANGEMENT //
    /////////////////

    team_box.append(&team_dropdown);
    team_box.append(&team_logo_container);

    team_frame.set_child(Some(&team_box));
    
    team_frame
}

fn get_team_icon(path: Option<&str>) -> gtk::Image {
    if let Some(path) = path {
        let path = crate::fs::from_web_path(path);
        util::load_image(&path, 200, 200)
    } else {
        gtk::Image::from_icon_name("image-missing")
    }
}

fn set_round_progress_box(shared_state: SharedState, round_progress_box: gtk::Box) {
    let state = shared_state.lock().unwrap();
    for (i, round) in state.current_match.rounds.iter().enumerate() {
        round_progress_box.append(&make_round_frame(shared_state.clone(), &state, round, i));
    }
}

fn make_round_frame(shared_state: SharedState, state: &AppState, round: &models::Round, round_index: usize) -> gtk::Frame {

    //////////////////
    // DECLARATIONS //
    //////////////////
    
    let gamemode_names = state.settings.gamemodes.iter().map(|g| g.name.clone()).collect();
    let gamemode_index = util::index_of_or_none(&gamemode_names, &round.gamemode);
    let gamemode = gamemode_index.and_then(|index| state.settings.gamemodes.get(index));
    let gamemode_model = util::get_model_with_none(&gamemode_names);
    
    let map_names = gamemode
        .and_then(|gamemode| Some(gamemode.maps.iter().map(|m| m.name.clone()).collect()))
        .unwrap_or(Vec::new());
    let map_model = util::get_model_with_none(&map_names);

    let round_frame = util::make_frame(format!("Round {}", round_index + 1).as_str(), 12, 12, 12, 12);
    let round_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);

    let round_info_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
    let gamemode_label = gtk::Label::new(Some("Gamemode"));
    let gamemode_dropdown = gtk::DropDown::new(Some(gamemode_model), gtk::Expression::NONE);
    let map_label = gtk::Label::new(Some("Map"));
    let map_dropdown = gtk::DropDown::new(Some(map_model), gtk::Expression::NONE);

    let scores_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
    let team1_score_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
    let team1_score_label = gtk::Label::new(Some("Team 1 Score"));
    let team1_score_entry = util::make_spin_button(12, 12, 12, 12);
    let team2_score_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
    let team2_score_label = gtk::Label::new(Some("Team 2 Score"));
    let team2_score_entry = util::make_spin_button(12, 12, 12, 12);

    let completed_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    let completed_label = gtk::Label::new(Some("Completed "));
    let completed_switch = gtk::Switch::builder().build();

    ////////////////////
    // INITIALIZATION //
    ////////////////////
    
    if let Some(index) = gamemode_index {
        gamemode_dropdown.set_selected(index as u32 + 1);
    } else {
        gamemode_dropdown.set_selected(0);
        // TODO: set gamemode in app state to none
    }

    if let Some(index) = util::index_of_or_none(&map_names, &round.map) {
        map_dropdown.set_selected(index as u32 + 1);
    } else {
        map_dropdown.set_selected(0);
        // TODO: set map in app state to none
    }

    team1_score_entry.set_value(round.team1_score as f64);
    team2_score_entry.set_value(round.team2_score as f64);
    completed_switch.set_state(round.completed);

    /////////////////
    // CONNECTIONS //
    /////////////////

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

    /////////////////
    // ARRANGEMENT //
    /////////////////

    round_info_box.append(&gamemode_label);
    round_info_box.append(&gamemode_dropdown);
    round_info_box.append(&map_label);
    round_info_box.append(&map_dropdown);

    team1_score_box.append(&team1_score_label);
    team1_score_box.append(&team1_score_entry);
    team2_score_box.append(&team2_score_label);
    team2_score_box.append(&team2_score_entry);

    scores_box.append(&team1_score_box);
    scores_box.append(&team2_score_box);

    completed_box.append(&completed_label);
    completed_box.append(&completed_switch);

    round_box.append(&round_info_box);
    round_box.append(&scores_box);
    round_box.append(&completed_box);

    round_frame.set_child(Some(&round_box));
    
    round_frame
}
