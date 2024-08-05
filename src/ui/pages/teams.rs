use gtk::prelude::*;
use gtk::glib::{clone, closure_local};
use gtk::glib;

use crate::{models, ui::{EntryWindowField, components::refresh_box}, SharedState};

pub fn build_box(window: &gtk::ApplicationWindow, shared_state: SharedState) -> refresh_box::RefreshBox {
    let refresh_box = refresh_box::RefreshBox::new();
    refresh_box.set_orientation(gtk::Orientation::Vertical);

    let (teams_list_box, teams_list) = crate::ui::make_list();
    let add_team_button = crate::ui::make_button("Add Team");
    let remove_team_button = crate::ui::make_button("Remove Team");

    let (players_list_box, players_list_window) = crate::ui::make_list();
    let add_player_button = crate::ui::make_button("Add Player");
    let remove_player_button = crate::ui::make_button("Remove Player");
    
    teams_list_box.connect_row_selected(clone!(
        #[strong] shared_state,
        #[weak] players_list_box,
        move |_, row| {
            if let Some(row) = row {
                let team_name = crate::ui::get_string_from_box_row(&row).unwrap();
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
            crate::ui::open_entry_window(
                &window,
                "New Team",
                vec![
                    EntryWindowField::Text { label: String::from("Name"), prefill: None },
                    EntryWindowField::File { label: String::from("Icon"), filters: vec![
                        (String::from("Image"), vec![String::from("*.png"), String::from("*.jpg"), String::from("*.jpeg")])
                    ] },
                ],
                Box::new(clone!(
                    #[strong] shared_state,
                    move |results| {
                        let mut state = shared_state.lock().unwrap();
                        let team_name = results.get("Name").unwrap_or(&None);
                        let team_icon = results.get("Icon").unwrap_or(&None);
                        let new_team = models::Team::new(
                            &team_name.as_ref().unwrap_or(&String::from("New String")),
                            team_icon.clone(),
                            vec![]
                        );
                        teams_list_box.append(&make_team_row(&new_team));
                        state.division.teams.push(new_team);
                    }
                )
            ));
        }
    ));

    remove_team_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] teams_list_box,
        #[weak] players_list_box,
        move |_| {
            let selected_row = teams_list_box.selected_row();
            if let Some(selected_row) = selected_row {
                let row_index = selected_row.index() as usize;
                shared_state.lock().unwrap().division.teams.remove(row_index);
                correct_bracket(shared_state.clone(), row_index);
                teams_list_box.remove(&selected_row);
                set_team_info(&players_list_box, None);
            }
        }
    ));

    add_player_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] teams_list_box,
        #[weak] players_list_box,
        #[weak] window,
        move |_| {
            let state = shared_state.lock().unwrap();
            crate::ui::open_entry_window(
                &window,
                "New Player",
                vec![
                    EntryWindowField::Text { label: String::from("Name"), prefill: None },
                    EntryWindowField::DropDown {
                        label: String::from("Role"),
                        options: state.settings.roles.iter().map(|role| role.name.clone()).collect(),
                        prefill: None,
                    },
                    EntryWindowField::DropDown {
                        label: String::from("Hero"),
                        options: state.settings.characters.iter().map(|character| character.name.clone()).collect(),
                        prefill: None,
                    },
                ],
                Box::new(clone!(
                    #[strong] shared_state,
                    move |results| {
                        let mut state = shared_state.lock().unwrap();
                        let player_name = results.get("Name").unwrap_or(&None);
                        let player_role = results.get("Role").unwrap_or(&None);
                        let player_hero = results.get("Hero").unwrap_or(&None);
                        let new_player = models::Player::new(
                            &player_name.as_ref().unwrap_or(&String::from("New Player")),
                            &player_role.as_ref().unwrap_or(&String::from("Unknown")),
                            &player_hero.as_ref().unwrap_or(&String::from("Unknown")),
                        );
                        let team_row = teams_list_box.selected_row();
                        if let Some(team_row) = team_row {
                            let team_name = crate::ui::get_string_from_box_row(&team_row).unwrap();
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
                let team_name = crate::ui::get_string_from_box_row(&selected_team_row).unwrap();
                let mut state = shared_state.lock().unwrap();
                let team = state.division.teams.iter_mut().find(|team| team.name == team_name).unwrap();
                let selected_player_row = players_list_box.selected_row();
                if let Some(selected_player_row) = selected_player_row {
                    let player_name = crate::ui::get_string_from_box_row(&selected_player_row).unwrap();
                    team.players.retain(|player| player.name != player_name);
                    set_team_info(&players_list_box, Some(team));
                }
            }
        }
    ));

    refresh_box.connect_closure(
        "refresh-status",
        false,
        closure_local!(
            #[strong] shared_state,
            #[weak] teams_list_box,
            #[weak] players_list_box,
            move |_box: refresh_box::RefreshBox, new_status: bool| {
                if new_status {
                    init_teams(&teams_list_box, &shared_state);
                    set_team_info(&players_list_box, None);
                } else {
                    teams_list_box.remove_all();
                }
            }
        )
    );

    refresh_box.append(&teams_list);
    refresh_box.append(&add_team_button);
    refresh_box.append(&remove_team_button);
    refresh_box.append(&players_list_window);
    refresh_box.append(&add_player_button);
    refresh_box.append(&remove_player_button);

    refresh_box
}

fn init_teams(list_box: &gtk::ListBox, shared_state: &SharedState) {
    let state = shared_state.lock().unwrap();
    for team in &state.division.teams {
        list_box.append(&make_team_row(team));
    }
}

fn make_team_row(team: &models::Team) -> gtk::Box {
    let team_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let team_label = crate::ui::make_label(&team.name);
    let team_icon = match &team.icon {
        Some(icon) => crate::ui::load_image(icon, 30, 30),
        None => gtk::Image::from_icon_name("image-missing"), // TODO: Implement missing icon
    };

    team_box.append(&team_label);
    team_box.append(&team_icon);

    team_box
}

fn set_team_info(players_list_box: &gtk::ListBox, team_info: Option<&models::Team>) {
    players_list_box.remove_all();

    if let Some(team_info) = team_info {
        for player in &team_info.players {
            players_list_box.append(&make_player_row(player));
        }
    }
}

fn make_player_row(player: &models::Player) -> gtk::Box {
    let player_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let player_label = crate::ui::make_label(&player.name);
    let role_label = crate::ui::make_label(&player.role);
    let hero_label = crate::ui::make_label(&player.character);

    player_box.append(&player_label);
    player_box.append(&role_label);
    player_box.append(&hero_label);

    player_box
}

fn correct_bracket(shared_state: SharedState, removed_team: usize) {
    let mut state = shared_state.lock().unwrap();
    for col in &mut state.division.bracket {
        for row in col {
            match row {
                Some(cell) if *cell == removed_team => *row = None,
                Some(cell) if *cell > removed_team => *row = Some(*cell - 1),
                _ => (),
            }
        }
    }
}
