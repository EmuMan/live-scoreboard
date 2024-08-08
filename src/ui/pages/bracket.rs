use gtk::prelude::*;
use gtk::glib::{clone, closure_local};
use gtk::glib;

use crate::{ui::{util, components::refresh_box::RefreshBox}, SharedState, AppState};

pub fn build_box(_window: &gtk::ApplicationWindow, shared_state: SharedState) -> RefreshBox {

    //////////////////
    // DECLARATIONS //
    //////////////////

    let refresh_box = RefreshBox::new();
    refresh_box.set_orientation(gtk::Orientation::Vertical);

    let bracket_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);

    /////////////////
    // CONNECTIONS //
    /////////////////

    refresh_box.connect_closure(
        "refresh-status",
        false,
        closure_local!(
            #[strong] bracket_box,
            move |_: RefreshBox, new_status: bool| {
                if new_status {
                    init_bracket(&bracket_box, shared_state.clone());
                } else {
                    util::clear_box(&bracket_box);
                }
            }
        )
    );

    /////////////////
    // ARRANGEMENT //
    /////////////////
    
    refresh_box.append(&bracket_box);

    refresh_box
}

pub fn init_bracket(bracket_box: &gtk::Box, shared_state: SharedState) {
    // first column, 4 matchups
    let column1 = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    column1.append(&util::make_label("Quarterfinals", 12, 12, 12, 12));
    for i in 0..4 {
        let matchup = make_matchup(shared_state.clone(), 0, i);
        column1.append(&matchup);
    };

    // second column, 2 matchups
    let column2 = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    column2.append(&util::make_label("Semifinals", 12, 12, 12, 12));
    for i in 0..2 {
        let matchup = make_matchup(shared_state.clone(), 1, i);
        column2.append(&matchup);
    };

    // third column, 1 matchup
    let column3 = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    column3.append(&util::make_label("Finals", 12, 12, 12, 12));
    let matchup = make_matchup(shared_state.clone(), 2, 0);
    column3.append(&matchup);

    bracket_box.append(&column1);
    bracket_box.append(&column2);
    bracket_box.append(&column3);
}

fn make_matchup(shared_state: SharedState, col: u32, row: u32) -> gtk::Box {

    //////////////////
    // DECLARATIONS //
    //////////////////

    let matchup_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);

    let team_names = shared_state.lock().unwrap().team_names();
    let team_names_model = util::get_model_with_none(&team_names);

    let team1_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    let team1_dropdown = gtk::DropDown::new(Some(team_names_model.clone()), gtk::Expression::NONE);
    let team1_score = util::make_spin_button(0, 0, 12, 12);

    let team2_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    let team2_dropdown = gtk::DropDown::new(Some(team_names_model.clone()), gtk::Expression::NONE);
    let team2_score = util::make_spin_button(0, 0, 12, 12);

    let winner_label = util::make_label("Winner:", 6, 12, 12, 12);
    let winner_dropdown = gtk::DropDown::new(Some(team_names_model), gtk::Expression::NONE);

    ////////////////////
    // INITIALIZATION //
    ////////////////////

    {
        let mut state = shared_state.lock().unwrap();
        let matchup = get_matchup_ref_mut(&mut state, col, row);
        
        team1_dropdown.set_selected(matchup.as_ref()
            .and_then(|m| m.team1)
            .map(|t| t + 1)
            .unwrap_or(0) as u32);
    
        team1_score.set_value(matchup.as_ref()
            .map(|m| m.team1_score as f64)
            .unwrap_or(0.0));
    
        team2_dropdown.set_selected(matchup.as_ref()
            .and_then(|m| m.team2)
            .map(|t| t + 1)
            .unwrap_or(0) as u32);
        
        team2_score.set_value(matchup.as_ref()
            .map(|m| m.team2_score as f64)
            .unwrap_or(0.0));

        winner_dropdown.set_selected(matchup.as_ref()
            .and_then(|m| m.winner)
            .map(|t| t + 1)
            .unwrap_or(0) as u32);
    }

    /////////////////
    // CONNECTIONS //
    /////////////////
    
    team1_dropdown.connect_selected_notify(clone!(
        #[strong] shared_state,
        move |dropdown| {
            let mut state = shared_state.lock().unwrap();
            if let Some(matchup) = get_matchup_ref_mut(&mut state, col, row) {
                let team1 = dropdown.selected();
                matchup.team1 = if team1 == 0 { None } else { Some((team1 - 1) as usize) };
            }
        }
    ));

    team1_score.connect_value_changed(clone!(
        #[strong] shared_state,
        move |spin_button| {
            let mut state = shared_state.lock().unwrap();
            if let Some(matchup) = get_matchup_ref_mut(&mut state, col, row) {
                matchup.team1_score = spin_button.value() as usize;
            }
        }
    ));

    team2_dropdown.connect_selected_notify(clone!(
        #[strong] shared_state,
        move |dropdown| {
            let mut state = shared_state.lock().unwrap();
            if let Some(matchup) = get_matchup_ref_mut(&mut state, col, row) {
                let team2 = dropdown.selected();
                matchup.team2 = if team2 == 0 { None } else { Some((team2 - 1) as usize) };
            }
        }
    ));

    team2_score.connect_value_changed(clone!(
        #[strong] shared_state,
        move |spin_button| {
            let mut state = shared_state.lock().unwrap();
            if let Some(matchup) = get_matchup_ref_mut(&mut state, col, row) {
                matchup.team2_score = spin_button.value() as usize;
            }
        }
    ));

    winner_dropdown.connect_selected_notify(clone!(
        #[strong] shared_state,
        move |dropdown| {
            let mut state = shared_state.lock().unwrap();
            if let Some(matchup) = get_matchup_ref_mut(&mut state, col, row) {
                let winner = dropdown.selected();
                matchup.winner = if winner == 0 { None } else { Some((winner - 1) as usize) };
            }
        }
    ));

    /////////////////
    // ARRANGEMENT //
    /////////////////

    team1_box.append(&team1_dropdown);
    team1_box.append(&team1_score);

    team2_box.append(&team2_dropdown);
    team2_box.append(&team2_score);

    matchup_box.append(&team1_box);
    matchup_box.append(&team2_box);
    matchup_box.append(&winner_label);
    matchup_box.append(&winner_dropdown);

    matchup_box
}

fn get_matchup_ref_mut(state: &mut AppState, col: u32, row: u32) -> Option<&mut crate::models::Matchup> {
    let bracket = &mut state.division.bracket;
    bracket.get_mut(col as usize)
        .and_then(|col| col.get_mut(row as usize))
}
