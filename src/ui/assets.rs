use gtk::prelude::*;
use gtk::glib::clone;
use gtk::glib;

use crate::{models, SharedState};

pub fn build_box(window: &gtk::ApplicationWindow, shared_state: SharedState) -> gtk::Box {
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let (assets_list_box, assets_list) = super::make_list();
    let add_asset_button = super::make_button("Add Asset");
    let remove_asset_button = super::make_button("Remove Asset");

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
                let asset_name = super::get_string_from_box_row(&row).unwrap();
                println!("Finding for asset: {}", asset_name);
                let state = shared_state.lock().unwrap();
                let asset = state.assets.iter().find(|asset| asset.name == asset_name);
                if let Some(asset) = asset {
                    println!("Found asset: {}", asset.name);
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
            super::make_entry_window(
                &window,
                "New Asset",
                vec![String::from("Name"), String::from("Path")],
                Box::new(clone!(
                    #[strong] shared_state,
                    move |results| {
                        let mut state = shared_state.lock().unwrap();
                        let new_asset = models::Asset::new(
                            results.get("Name").unwrap_or(&String::from("None")),
                            results.get("Path").unwrap_or(&String::from("/")),
                        );
                        assets_list_box.append(&make_asset_box(&new_asset));
                        state.assets.push(new_asset);
                    }
                )
            ));
        }
    ));

    remove_asset_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] assets_list_box,
        move |_| {
            let selected_row = assets_list_box.selected_row();
            if let Some(selected_row) = selected_row {
                let row_index = selected_row.index() as usize;
                shared_state.lock().unwrap().assets.remove(row_index);
                assets_list_box.remove(&selected_row);
            }
        }
    ));

    gtk_box.connect_visible_notify(clone!(
        #[strong] shared_state,
        #[weak] assets_list_box,
        move |gtk_box| {
            if gtk_box.is_visible() {
                let state = shared_state.lock().unwrap();
                assets_list_box.remove_all();
                for team in &state.assets {
                    assets_list_box.append(&make_asset_box(team));
                }
            }
        }
    ));

    gtk_box.append(&assets_list);
    gtk_box.append(&add_asset_button);
    gtk_box.append(&remove_asset_button);
    gtk_box.append(&picture_container);

    gtk_box
}

fn make_asset_box(asset: &models::Asset) -> gtk::Box {
    let asset_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let name_label = super::make_label(&asset.name);
    let path_label = super::make_label(&asset.path);

    asset_box.append(&name_label);
    asset_box.append(&path_label);

    asset_box
}
