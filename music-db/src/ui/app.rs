use gtk4::prelude::*;
use gtk4::Application;

use super::window;

const APP_ID: &str = "com.musicdb.app";

/// Iniciar GStreamer y la aplicacion GTK.
pub fn run() {
    gstreamer::init().expect("No se pudo iniciar GStreamer");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(move |app| {
        window::build_window(app);
    });

    app.run();
}
