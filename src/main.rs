extern crate gio;

use gio::prelude::*;

use std::env::args;

pub mod ui_interactions;

fn main() {
	let application = gtk::Application::new(
		Some("arduino.application"),
		Default::default(),
	)
	.expect("Initialization failed...");

	application.connect_activate(|app| {
		ui_interactions::build_ui(app);
	});

	application.run(&args().collect::<Vec<_>>());

}