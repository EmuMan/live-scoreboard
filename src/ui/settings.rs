use std::rc::Rc;
use std::cell::{Cell, RefCell};

use gtk::prelude::*;
use gtk::{glib, Button, Label, Box};
use glib::clone;
use tokio::sync::oneshot;

use crate::{webserver, runtime};

pub fn build_box() -> Box {
    let server: Rc<Cell<Option<tokio::task::JoinHandle<()>>>> = Rc::new(Cell::new(None));
    let server_stop_tx: Rc<RefCell<Option<oneshot::Sender<()>>>> = Rc::new(RefCell::new(None));

    let gtk_box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let label = Label::builder()
        .label("Webserver")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let start_ws_button = Button::builder()
        .label("Start Webserver")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let stop_ws_button = Button::builder()
        .label("Stop Webserver")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    start_ws_button.connect_clicked(clone!(
        #[strong]
        server,
        #[strong]
        server_stop_tx,
        move |button| {
            println!("Starting webserver...");
            let (tx, rx) = oneshot::channel::<()>();
            let handle = runtime().spawn(
                webserver::create_and_run_webserver("templates/**/*", "0.0.0.0:3000", rx)
            );
            println!("Webserver started!");
            server.set(Some(handle));
            *server_stop_tx.borrow_mut() = Some(tx);
            button.set_sensitive(false);
        }
    ));

    stop_ws_button.connect_clicked(clone!(
        #[strong]
        server_stop_tx,
        #[strong]
        server,
        #[weak]
        start_ws_button,
        move |_| {
            if let Some(handle) = server.take() {
                if let Some(tx) = server_stop_tx.borrow_mut().take() {
                    println!("Stopping webserver...");
                    tx.send(()).unwrap();
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

    gtk_box.append(&label);
    gtk_box.append(&start_ws_button);
    gtk_box.append(&stop_ws_button);

    gtk_box
}
