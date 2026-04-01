use gtk4::prelude::*;
use gtk4::Label;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Button;
use std::rc::Rc;
use std::cell::RefCell;

pub fn inicia_vista(){
	let app = Application::builder()
		.application_id("Prueba GTK en Rust")
		.build();
	app.connect_activate(|app|{
		let window = ApplicationWindow::new(app);
		window.set_default_size(500, 500);
		window.set_title(Some("GTK Window"));

		let rc_count = Rc::new(RefCell::new(0));
		let rc_count_clone = rc_count.clone();

		let label = Label::new(Some("Contador: "));

		let label_clone = label.clone();

		let button = Button::new();
		button.set_halign(gtk4::Align::Start);
		button.set_label("Contador");
		button.connect_clicked(move | button | {
			*rc_count_clone.borrow_mut() += 1;
			label_clone.set_label(&format!("Contador: {}", *rc_count_clone.borrow()));
		});

		let layout = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

		layout.append(&button);
		layout.append(&label);

		window.set_child(Some(&layout));

		window.show();
	});

	app.run();
}
