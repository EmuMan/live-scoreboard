use gtk::prelude::*;
use gtk::glib::clone;
use gtk::glib;

use crate::{models, SharedState};

pub fn build_box(window: &gtk::ApplicationWindow, shared_state: SharedState) -> gtk::Box {
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let (teams_list_box, teams_list) = super::make_list();
    let add_team_button = super::make_button("Add Team");
    let remove_team_button = super::make_button("Remove Team");

    let (players_list_box, players_list_window) = super::make_list();
    let add_player_button = super::make_button("Add Player");
    let remove_player_button = super::make_button("Remove Player");
    
    teams_list_box.connect_row_selected(clone!(
        #[strong] shared_state,
        #[weak] players_list_box,
        move |_, row| {
            if let Some(row) = row {
                let team_name = super::get_string_from_label_row(&row).unwrap();
                let state = shared_state.lock().unwrap();
                let team = state.division.teams.iter().find(|team| team.name == team_name);
                if let Some(team) = team {
                    set_team_info(&players_list_box, Some(team));
                }
            }
        }
    ));

    add_team_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] teams_list_box,
        #[weak] window,
        move |_| {
            super::make_entry_window(
                &window,
                "New Team",
                vec![String::from("Name")],
                Box::new(clone!(
                    #[strong] shared_state,
                    move |results| {
                        let mut state = shared_state.lock().unwrap();
                        let new_team = models::Team::new(
                            results.get("Name").unwrap_or(&String::from("None")),
                            vec![]
                        );
                        teams_list_box.append(&gtk::Label::new(Some(&new_team.name)));
                        state.division.teams.push(new_team);
                    }
                )
            ));
        }
    ));

    remove_team_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] teams_list_box,
        move |_| {
            let selected_row = teams_list_box.selected_row();
            if let Some(selected_row) = selected_row {
                let team_name = super::get_string_from_label_row(&selected_row).unwrap();
                let mut state = shared_state.lock().unwrap();
                if state.division.teams.iter().any(|team| team.name == team_name) {
                    state.division.teams.retain(|team| team.name != team_name);
                    teams_list_box.remove(&selected_row);
                }
                set_team_info(&teams_list_box, None);
            }
        }
    ));

    add_player_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] teams_list_box,
        #[weak] players_list_box,
        #[weak] window,
        move |_| {
            super::make_entry_window(
                &window,
                "New Player",
                vec![String::from("Name"), String::from("Role"), String::from("Hero")],
                Box::new(clone!(
                    #[strong] shared_state,
                    move |results| {
                        let mut state = shared_state.lock().unwrap();
                        let new_player = models::Player::new(
                            results.get("Name").unwrap_or(&String::from("None")),
                            results.get("Role").unwrap_or(&String::from("None")),
                            results.get("Hero").unwrap_or(&String::from("None")),
                        );
                        let team_row = teams_list_box.selected_row();
                        if let Some(team_row) = team_row {
                            let team_name = super::get_string_from_label_row(&team_row).unwrap();
                            let team = state.division.teams.iter_mut().find(|team| team.name == team_name);
                            if let Some(team) = team {
                                team.players.push(new_player);
                                set_team_info(&players_list_box, Some(team));
                            }
                        }
                    }
                ))
            );
        }
    ));

    remove_player_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] teams_list_box,
        #[weak] players_list_box,
        move |_| {
            if let Some(selected_team_row) = teams_list_box.selected_row() {
                let team_name = super::get_string_from_label_row(&selected_team_row).unwrap();
                let mut state = shared_state.lock().unwrap();
                let team = state.division.teams.iter_mut().find(|team| team.name == team_name).unwrap();
                let selected_player_row = players_list_box.selected_row();
                if let Some(selected_player_row) = selected_player_row {
                    let player_name = super::get_string_from_box_row(&selected_player_row).unwrap();
                    team.players.retain(|player| player.name != player_name);
                    set_team_info(&players_list_box, Some(team));
                }
            }
        }
    ));

    gtk_box.append(&teams_list);
    gtk_box.append(&add_team_button);
    gtk_box.append(&remove_team_button);
    gtk_box.append(&players_list_window);
    gtk_box.append(&add_player_button);
    gtk_box.append(&remove_player_button);

    gtk_box
}

fn set_team_info(list_box: &gtk::ListBox, team_info: Option<&models::Team>) {
    list_box.remove_all();

    if let Some(team_info) = team_info {
        for player in &team_info.players {
            let player_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();
    
            let player_label = super::make_label(&player.name);
            let hero_label = super::make_label(&player.hero);
            let role_label = super::make_label(&player.role);
    
            player_box.append(&player_label);
            player_box.append(&hero_label);
            player_box.append(&role_label);
    
            list_box.append(&player_box);
        }
    }
}
