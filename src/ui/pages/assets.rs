use gtk::prelude::*;
use gtk::glib::{self, clone, closure_local};

use crate::{models, ui::components::refresh_box, SharedState};

pub fn build_box(window: &gtk::ApplicationWindow, shared_state: SharedState) -> refresh_box::RefreshBox {
    let refresh_box = refresh_box::RefreshBox::new();
    refresh_box.set_orientation(gtk::Orientation::Vertical);

    let (assets_list_box, assets_list) = crate::ui::make_list();
    let add_asset_button = crate::ui::make_button("Add Asset");
    let remove_asset_button = crate::ui::make_button("Remove Asset");

    let picture_container = gtk::Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    assets_list_box.connect_row_selected(clone!(
        #[strong] shared_state,
        #[weak] picture_container,
        move |_, row| {
            if let Some(row) = row {
                let asset_name = crate::ui::get_string_from_box_row(&row).unwrap();
                let state = shared_state.lock().unwrap();
                let asset = state.assets.iter().find(|asset| asset.name == asset_name);
                if let Some(asset) = asset {
                    let image = gtk::Image::builder()
                        .file(&asset.path)
                        .build();
                    image.set_size_request(200, 200);
                    if let Some(child) = picture_container.first_child() {
                        picture_container.remove(&child);
                    }
                    picture_container.append(&image);
                }
            }
        }
    ));

    add_asset_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] assets_list_box,
        #[weak] window,
        move |_| {
            crate::ui::open_entry_window(
                &window,
                "New Asset",
                vec![
                    crate::ui::EntryWindowField::Text { label: String::from("Name"), prefill: None },
                    crate::ui::EntryWindowField::File { label: String::from("File"), filters: Vec::new() },
                ],
                Box::new(clone!(
                    #[strong] shared_state,
                    move |results| {
                        let mut state = shared_state.lock().unwrap();
                        let asset_name = results.get("Name").unwrap_or(&None);
                        let asset_file = results.get("File").unwrap_or(&None);
                        let new_asset = models::Asset::new(
                            &asset_name.as_ref().unwrap_or(&String::from("New Asset")),
                            &asset_file.as_ref().unwrap_or(&String::from("(none)")),
                        );
                        assets_list_box.append(&make_asset_row(&new_asset));
                        state.assets.push(new_asset);
                    }
                )
            ));
        }
    ));

    remove_asset_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] assets_list_box,
        #[weak] picture_container,
        move |_| {
            let selected_row = assets_list_box.selected_row();
            if let Some(selected_row) = selected_row {
                let row_index = selected_row.index() as usize;
                shared_state.lock().unwrap().assets.remove(row_index);
                assets_list_box.remove(&selected_row);
                if let Some(child) = picture_container.first_child() {
                    picture_container.remove(&child);
                }
            }
        }
    ));

    refresh_box.connect_closure(
        "refresh-status",
        false,
        closure_local!(
            #[strong] shared_state,
            #[weak] assets_list_box,
            move |_box: refresh_box::RefreshBox, new_status: bool| {
                if new_status {
                    let state = shared_state.lock().unwrap();
                    for asset in &state.assets {
                        assets_list_box.append(&make_asset_row(asset));
                    }
                } else {
                    assets_list_box.remove_all();
                }
            }
        )
    );

    refresh_box.append(&assets_list);
    refresh_box.append(&add_asset_button);
    refresh_box.append(&remove_asset_button);
    refresh_box.append(&picture_container);

    refresh_box
}

fn make_asset_row(asset: &models::Asset) -> gtk::Box {
    let asset_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let name_label = crate::ui::make_label(&asset.name);
    let path_label = crate::ui::make_label(&asset.path);

    asset_box.append(&name_label);
    asset_box.append(&path_label);

    asset_box
}
