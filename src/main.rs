use gtk::prelude::*;
use gtk::{glib, Application};

use live_scoreboard::ui;

const APP_ID: &str = "net.emuman.LiveScoreboard";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(ui::build_ui);

    app.run()
}
