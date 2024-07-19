pub mod settings;
pub mod teams;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub fn build_ui(app: &Application) {
    let notebook = gtk::Notebook::builder()
        .scrollable(true)
        .build();

    let teams_box = teams::build_box();
    let teams_label = gtk::Label::new(Some("Teams"));
    notebook.append_page(&teams_box, Some(&teams_label));

    let settings_box = settings::build_box();
    let settings_label = gtk::Label::new(Some("Settings"));
    notebook.append_page(&settings_box, Some(&settings_label));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Live Scoreboard")
        .default_width(500)
        .default_height(1000)
        .child(&notebook)
        .build();

    window.present();
}
