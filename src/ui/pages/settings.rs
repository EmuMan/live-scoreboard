use std::rc::Rc;
use std::cell::{Cell, RefCell};

use gtk::prelude::*;
use gtk::glib;
use glib::clone;
use tokio::sync::oneshot;

use crate::{webserver, ui::components::refresh_box, runtime, SharedState};

pub fn build_box(
    primary_window: &gtk::ApplicationWindow,
    shared_state: SharedState,
) -> refresh_box::RefreshBox {
    let server: Rc<Cell<Option<tokio::task::JoinHandle<()>>>> = Rc::new(Cell::new(None));
    let server_stop_tx: Rc<RefCell<Option<oneshot::Sender<()>>>> = Rc::new(RefCell::new(None));

    let refresh_box = refresh_box::RefreshBox::new();
    refresh_box.set_orientation(gtk::Orientation::Vertical);

    let webserver_label = crate::ui::make_label("Webserver");
    let start_ws_button = crate::ui::make_button("Start Webserver");
    let stop_ws_button = crate::ui::make_button("Stop Webserver");
    
    let config_label = crate::ui::make_label("Config");
    let open_config_button = crate::ui::make_button("Open Config");
    let save_config_button = crate::ui::make_button("Save Config");

    start_ws_button.connect_clicked(clone!(
        #[strong] server,
        #[strong] server_stop_tx,
        #[strong] shared_state,
        move |button| {
            println!("Starting webserver...");
            let (tx, rx) = oneshot::channel::<()>();
            let handle = runtime().spawn(
                webserver::create_and_run_webserver(
                    "templates/**/*",
                    "0.0.0.0:3000",
                    rx,
                    shared_state.clone()
                )
            );
            println!("Webserver started!");
            server.set(Some(handle));
            *server_stop_tx.borrow_mut() = Some(tx);
            button.set_sensitive(false);
        }
    ));

    stop_ws_button.connect_clicked(clone!(
        #[strong] server_stop_tx,
        #[strong] server,
        #[weak] start_ws_button,
        move |_| {
            if let Some(handle) = server.take() {
                if let Some(tx) = server_stop_tx.borrow_mut().take() {
                    println!("Stopping webserver...");
                    tx.send(()).unwrap_or_else(|err| {
                        println!("Webserver exited with error: {:?}", err);
                    });
                    handle.abort();
                    println!("Webserver stopped!");
                } else {
                    println!("Failed to stop webserver: no stop channel!");
                }
            } else {
                println!("Failed to stop webserver: no handle!");
            }
            start_ws_button.set_sensitive(true);
        }
    ));

    open_config_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] primary_window,
        move |_| {
            println!("Opening config file...");
            crate::fs::open_config_file(&primary_window, shared_state.clone());
        }
    ));

    save_config_button.connect_clicked(clone!(
        #[strong] shared_state,
        #[weak] primary_window,
        move |_| {
            println!("Saving config file...");
            crate::fs::save_config_file(&primary_window, shared_state.clone());
        }
    ));

    refresh_box.append(&webserver_label);
    refresh_box.append(&start_ws_button);
    refresh_box.append(&stop_ws_button);

    refresh_box.append(&config_label);
    refresh_box.append(&open_config_button);
    refresh_box.append(&save_config_button);

    refresh_box
}
