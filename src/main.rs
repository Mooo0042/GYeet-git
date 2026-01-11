mod ui;
mod config;
mod patcher;
mod proton;

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "com.gyeet.VotVPatcher";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    app.run();
}

