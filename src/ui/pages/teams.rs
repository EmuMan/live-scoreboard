use gtk::prelude::*;
use gtk::glib::{clone, closure_local};
use gtk::glib;

use crate::AppState;
use crate::ui::synced_list_box::{SyncedListBox, ConnectableList};
use crate::{models, ui::{util, entry_window::EntryWindowField, components::refresh_box::RefreshBox}, SharedState};

pub fn build_box(window: &gtk::ApplicationWindow, shared_state: SharedState) -> RefreshBox {

    //////////////////
    // DECLARATIONS //
    //////////////////

    let refresh_box = RefreshBox::new();
    refresh_box.set_orientation(gtk::Orientation::Vertical);

    let teams_label = util::make_label("Teams", 12, 12, 12, 12);
    let (teams_list_box, teams_list) = util::make_list(12, 12, 12, 12);
    let teams_buttons_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    let add_team_button = util::make_button("Add", 12, 12, 0, 0);
    let remove_team_button = util::make_button("Remove", 12, 12, 0, 0);
    let edit_team_button = util::make_button("Edit", 12, 12, 0, 0);
    let move_team_up_button = util::make_button("Move Up", 12, 12, 0, 0);
    let move_team_down_button = util::make_button("Move Down", 12, 12, 0, 0);

    let players_label = util::make_label("Players", 12, 12, 12, 12);
    let (players_list_box, players_list_window) = util::make_list(12, 12, 12, 12);
    let players_buttons_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    let add_player_button = util::make_button("Add", 12, 12, 0, 0);
    let remove_player_button = util::make_button("Remove", 12, 12, 0, 0);
    let edit_player_button = util::make_button("Edit", 12, 12, 0, 0);
    let move_player_up_button = util::make_button("Move Up", 12, 12, 0, 0);
    let move_player_down_button = util::make_button("Move Down", 12, 12, 0, 0);

    /////////////////
    // CONNECTIONS //
    /////////////////

    let teams_synced_list_box = SyncedListBox::new_shared(
        window.clone(),
        teams_list_box.clone(),
        shared_state.clone(),
        Box::new(move |team| {
            gtk::ListBoxRow::builder().child(&make_team_row(team)).build()
        }),
        Box::new(move |state| Some(&state.division.teams)),
        Box::new(move |state| Some(&mut state.division.teams)),
        Box::new(move |team| {
            vec![
                EntryWindowField::Text {
                    label: String::from("Name"),
                    prefill: team.as_ref().map(|team| team.name.clone())
                },
                EntryWindowField::File {
                    label: String::from("Icon"),
                    filters: Vec::new(),
                    prefill: team.as_ref().and_then(|team| team.icon.clone())
                },
            ]
        }),
        Box::new(move |fields, old_team_data| {
            let team_name = fields.get("Name").unwrap_or(&None);
            let team_icon = fields.get("Icon").unwrap_or(&None);
            models::Team::new(
                &team_name.as_ref().unwrap_or(&String::from("New Team")),
                team_icon.clone(),
                old_team_data.map(|team| team.players.clone()).unwrap_or(Vec::new()),
            )
        },
    ));

    teams_synced_list_box.connect_add_button(&add_team_button);
    teams_synced_list_box.connect_remove_button(&remove_team_button, Some(Box::new(correct_bracket)));
    teams_synced_list_box.connect_edit_button(&edit_team_button);
    teams_synced_list_box.connect_move_button(&move_team_up_button, -1, Some(Box::new(correct_bracket)));
    teams_synced_list_box.connect_move_button(&move_team_down_button, 1, Some(Box::new(correct_bracket)));

    let players_synced_list_box = SyncedListBox::new_shared(
        window.clone(),
        players_list_box.clone(),
        shared_state.clone(),
        Box::new(move |player| {
            gtk::ListBoxRow::builder().child(&make_player_row(player)).build()
        }),
        Box::new(clone!(
            #[strong] teams_list_box,
            move |state| {
                let selected_row = teams_list_box.selected_row();
                let index = selected_row.map(|row| row.index() as usize).unwrap_or(0);
                state.division.teams.get(index).map(|team| &team.players)
            }
        )),
        Box::new(clone!(
            #[strong] teams_list_box,
            move |state| {
                let selected_row = teams_list_box.selected_row();
                let index = selected_row.map(|row| row.index() as usize).unwrap_or(0);
                state.division.teams.get_mut(index).map(|team| &mut team.players)
            }
        )),
        Box::new(move |player| {
            let state = shared_state.lock().unwrap();
            vec![
                EntryWindowField::Text {
                    label: String::from("Name"),
                    prefill: player.as_ref().map(|player| player.name.clone())
                },
                EntryWindowField::DropDown {
                    label: String::from("Role"),
                    options: state.settings.roles.iter()
                        .map(|role| role.name.clone()).collect(),
                    prefill: player.as_ref().map(|player| player.role.clone())
                },
                EntryWindowField::DropDown {
                    label: String::from("Character"),
                    options: state.settings.characters.iter()
                        .map(|character| character.name.clone()).collect(),
                    prefill: player.as_ref().map(|player| player.character.clone())
                },
            ]
        }),
        Box::new(move |fields, _| {
            let player_name = fields.get("Name").unwrap_or(&None);
            let player_role = fields.get("Role").unwrap_or(&None);
            let player_character = fields.get("Character").unwrap_or(&None);
            models::Player::new(
                &player_name.as_ref().unwrap_or(&String::from("New Player")),
                &player_role.as_ref().unwrap_or(&String::from("(none)")),
                &player_character.as_ref().unwrap_or(&String::from("(none)")),
            )
        },
    ));

    players_synced_list_box.connect_add_button(&add_player_button);
    players_synced_list_box.connect_remove_button(&remove_player_button, None);
    players_synced_list_box.connect_edit_button(&edit_player_button);
    players_synced_list_box.connect_move_button(&move_player_up_button, -1, None);
    players_synced_list_box.connect_move_button(&move_player_down_button, 1, None);

    teams_list_box.connect_row_selected(clone!(
        #[strong] players_synced_list_box,
        move |_, _| {
            players_synced_list_box.lock().unwrap().populate();
        }
    ));

    refresh_box.connect_closure(
        "refresh-status",
        false,
        closure_local!(
            #[strong] teams_synced_list_box,
            #[weak] teams_list_box,
            #[weak] players_list_box,
            move |_box: RefreshBox, new_status: bool| {
                if new_status {
                    teams_synced_list_box.lock().unwrap().populate();
                } else {
                    teams_list_box.remove_all();
                    players_list_box.remove_all();
                }
            }
        )
    );

    /////////////////
    // ARRANGEMENT //
    /////////////////
    
    teams_buttons_box.append(&add_team_button);
    teams_buttons_box.append(&remove_team_button);
    teams_buttons_box.append(&edit_team_button);
    teams_buttons_box.append(&move_team_up_button);
    teams_buttons_box.append(&move_team_down_button);

    players_buttons_box.append(&add_player_button);
    players_buttons_box.append(&remove_player_button);
    players_buttons_box.append(&edit_player_button);
    players_buttons_box.append(&move_player_up_button);
    players_buttons_box.append(&move_player_down_button);

    refresh_box.append(&teams_label);
    refresh_box.append(&teams_list);
    refresh_box.append(&teams_buttons_box);
    refresh_box.append(&players_label);
    refresh_box.append(&players_list_window);
    refresh_box.append(&players_buttons_box);

    refresh_box
}

fn make_team_row(team: &models::Team) -> gtk::Box {
    let team_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let team_label = util::make_label(&team.name, 12, 12, 12, 12);
    let team_icon = match &team.icon {
        Some(path) => {
            let path = crate::fs::from_web_path(path);
            util::load_image(&path, 30, 30)
        },
        None => gtk::Image::from_icon_name("image-missing"), // TODO: Implement missing icon
    };

    team_box.append(&team_label);
    team_box.append(&team_icon);

    team_box
}

fn make_player_row(player: &models::Player) -> gtk::Box {
    let player_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let player_label = util::make_label(&player.name, 12, 12, 12, 12);
    let role_label = util::make_label(&player.role, 12, 12, 12, 12);
    let hero_label = util::make_label(&player.character, 12, 12, 12, 12);

    player_box.append(&player_label);
    player_box.append(&role_label);
    player_box.append(&hero_label);

    player_box
}

fn correct_bracket(state: &mut AppState, old_index: usize, new_index: Option<usize>) {
    // there is a better way to do this but i'm feeling kinda lazy rn tbh ngl
    for col in &mut state.division.bracket {
        for matchup in col {
            matchup.team1 = correct_index(matchup.team1, old_index, new_index);
            matchup.team2 = correct_index(matchup.team2, old_index, new_index);
            matchup.winner = correct_index(matchup.winner, old_index, new_index);
        }
    }
}

fn correct_index(index: Option<usize>, moved_from: usize, moved_to: Option<usize>) -> Option<usize> {
    match (index, moved_to) {
        (Some(index), Some(moved_to)) if index == moved_from => Some(moved_to),
        (Some(index), Some(moved_to)) if index > moved_from && index <= moved_to => Some(index - 1),
        (Some(index), Some(moved_to)) if index < moved_from && index >= moved_to => Some(index + 1),
        (Some(index), None) if index == moved_from => None,
        (Some(index), None) if index > moved_from => Some(index - 1),
        (Some(index), _) => Some(index),
        _ => None,
    }
}
