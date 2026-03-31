use gtk4::prelude::*;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Button;

pub fn inicia_vista(){
	let app = Application::builder()
		.application_id("Prueba GTK en Rust")
		.build();
	app.connect_activate(|app|{
		let window = ApplicationWindow::new(app);
		window.set_default_size(500, 500);
		window.set_title(Some("GTK Window"));

		window.show();
	});

	app.run();
}
