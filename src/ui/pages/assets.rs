use gtk::prelude::*;
use gtk::glib::{self, clone, closure_local};

use crate::ui::synced_list_box::{SyncedListBox, ConnectableList};
use crate::{models, fs, ui::{util, components::refresh_box::RefreshBox, entry_window::EntryWindowField}, SharedState};

pub fn build_box(window: &gtk::ApplicationWindow, shared_state: SharedState) -> RefreshBox {

    //////////////////
    // DECLARATIONS //
    //////////////////

    let refresh_box = RefreshBox::new();
    refresh_box.set_orientation(gtk::Orientation::Vertical);

    let assets_frame = util::make_frame("Assets", 12, 12, 12, 12);
    let assets_box = util::make_box(gtk::Orientation::Vertical, 12, 12, 12, 12);
    let (assets_list_box, assets_list_window) = util::make_list(12, 12, 12, 12);
    let assets_buttons_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    let add_asset_button = util::make_button("Add", 12, 12, 0, 0);
    let remove_asset_button = util::make_button("Remove", 12, 12, 0, 0);
    let edit_asset_button = util::make_button("Edit", 12, 12, 0, 0);
    let move_asset_up_button = util::make_button("Move Up", 12, 12, 0, 0);
    let move_asset_down_button = util::make_button("Move Down", 12, 12, 0, 0);

    let picture_container = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);

    /////////////////
    // CONNECTIONS //
    /////////////////

    let assets_synced_list_box = SyncedListBox::new_shared(
        window.clone(),
        assets_list_box.clone(),
        shared_state.clone(),
        Box::new(move |asset| {
            gtk::ListBoxRow::builder().child(&make_asset_row(asset)).build()
        }),
        Box::new(move |state| Some(&state.data.assets)),
        Box::new(move |state| Some(&mut state.data.assets)),
        Box::new(move |asset| {
            vec![
                EntryWindowField::Text {
                    label: String::from("Name"),
                    prefill: asset.as_ref().map(|asset| asset.name.clone())
                },
                EntryWindowField::File {
                    label: String::from("Path"),
                    filters: Vec::new(),
                    prefill: asset.as_ref().map(|asset| asset.path.clone())
                },
            ]
        }),
        Box::new(move |fields, _| {
            let asset_name = fields.get("Name").unwrap_or(&None);
            let asset_file = fields.get("Path").unwrap_or(&None);
            models::Asset::new(
                &asset_name.as_ref().unwrap_or(&String::from("New Asset")),
                &asset_file.as_ref().unwrap_or(&String::from("(none)")),
            )
        }),
    );

    assets_synced_list_box.connect_add_button(&add_asset_button);
    assets_synced_list_box.connect_remove_button(&remove_asset_button, None);
    assets_synced_list_box.connect_edit_button(&edit_asset_button);
    assets_synced_list_box.connect_move_button(&move_asset_up_button, -1, None);
    assets_synced_list_box.connect_move_button(&move_asset_down_button, 1, None);
    
    assets_list_box.connect_row_selected(clone!(
        #[strong] shared_state,
        #[weak] picture_container,
        move |_, selected_row| {
            let Some(selected_row) = selected_row else { return };
            let index = selected_row.index() as usize;
            let state = shared_state.lock().unwrap();
            let Some(asset) = state.data.assets.get(index) else { return };
            let Some(base_path) = state.get_base_path() else { return };
            let absolute_path = fs::from_relative_path(&base_path, &asset.path);
            println!("Loading image: {:?}", absolute_path);
            let image = util::load_image(&absolute_path, 200, 200);
            if let Some(child) = picture_container.first_child() {
                picture_container.remove(&child);
            }
            picture_container.append(&image);
        }
    ));

    refresh_box.connect_closure(
        "refresh-status",
        false,
        closure_local!(
            #[strong] assets_synced_list_box,
            #[weak] assets_list_box,
            move |_box: RefreshBox, new_status: bool| {
                if new_status {
                    assets_synced_list_box.lock().unwrap().populate();
                } else {
                    assets_list_box.remove_all();
                }
            }
        )
    );

    /////////////////
    // ARRANGEMENT //
    /////////////////

    assets_buttons_box.append(&add_asset_button);
    assets_buttons_box.append(&remove_asset_button);
    assets_buttons_box.append(&edit_asset_button);
    assets_buttons_box.append(&move_asset_up_button);
    assets_buttons_box.append(&move_asset_down_button);

    assets_box.append(&assets_list_window);
    assets_box.append(&assets_buttons_box);

    assets_frame.set_child(Some(&assets_box));

    refresh_box.append(&assets_frame);
    refresh_box.append(&picture_container);

    refresh_box
}

fn make_asset_row(asset: &models::Asset) -> gtk::Box {
    let asset_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let name_label = util::make_label(&asset.name, 12, 12, 12, 12);
    let path_label = util::make_label(&asset.path, 12, 12, 12, 12);

    asset_box.append(&name_label);
    asset_box.append(&path_label);

    asset_box
}
