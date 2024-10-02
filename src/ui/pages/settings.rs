use std::rc::Rc;
use std::cell::{Cell, RefCell};

use gtk::prelude::*;
use gtk::glib;
use glib::clone;
use tokio::sync::oneshot;

use crate::{webserver, fs, ui::{util, components::refresh_box::RefreshBox}, runtime, SharedState};

pub fn build_box(
    primary_window: &gtk::ApplicationWindow,
    shared_state: SharedState,
) -> RefreshBox {
    let server: Rc<Cell<Option<tokio::task::JoinHandle<()>>>> = Rc::new(Cell::new(None));
    let server_stop_tx: Rc<RefCell<Option<oneshot::Sender<()>>>> = Rc::new(RefCell::new(None));

    //////////////////
    // DECLARATIONS //
    //////////////////

    let refresh_box = RefreshBox::new();
    refresh_box.set_orientation(gtk::Orientation::Vertical);

    let webserver_frame = util::make_frame("Webserver", 12, 12, 12, 12);
    let webserver_buttons_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    let start_ws_button = util::make_button("Start Webserver", 12, 12, 0, 0);
    let stop_ws_button = util::make_button("Stop Webserver", 12, 12, 0, 0);
    
    let config_frame = util::make_frame("Config", 12, 12, 12, 12);
    let config_buttons_box = util::make_box(gtk::Orientation::Horizontal, 12, 12, 12, 12);
    let open_config_button = util::make_button("Open Config", 12, 12, 0, 0);
    let save_config_button = util::make_button("Save Config", 12, 12, 0, 0);

    /////////////////
    // CONNECTIONS //
    /////////////////

    start_ws_button.connect_clicked(clone!(
        #[strong] server,
        #[strong] server_stop_tx,
        #[strong] shared_state,
        move |button| {
            let Some(base_path) = shared_state.lock().unwrap().get_base_path() else {
                println!("Failed to start webserver: no config loaded!");
                return;
            };
            let templates_path = fs::from_relative_path(&base_path, "templates/**/*");
            println!("Starting webserver...");
            let (tx, rx) = oneshot::channel::<()>();
            let handle = runtime().spawn(
                webserver::create_and_run_webserver(
                    templates_path,
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

    /////////////////
    // ARRANGEMENT //
    /////////////////
    
    webserver_buttons_box.append(&start_ws_button);
    webserver_buttons_box.append(&stop_ws_button);

    config_buttons_box.append(&open_config_button);
    config_buttons_box.append(&save_config_button);

    webserver_frame.set_child(Some(&webserver_buttons_box));
    config_frame.set_child(Some(&config_buttons_box));
    
    refresh_box.append(&webserver_frame);
    refresh_box.append(&config_frame);

    refresh_box
}
