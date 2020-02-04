extern crate gtk;

use gtk::prelude::*;

use gtk::{ApplicationWindow ,Builder, Button, CheckButton, Entry, ButtonsType, DialogFlags, Orientation, Label, ListBox, ListBoxRow, MessageType, MessageDialog, Notebook, Window};

use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

mod network;
mod module;
use crate::ui_interactions::module::*;


fn message_box(flags: DialogFlags, msg_type: MessageType, btn_type: ButtonsType, text: &str) {
	let message = MessageDialog::new(None::<&Window>, 
		flags,
		msg_type,
		btn_type,
		text);
	message.run();
	message.hide();
}

fn list_row(entry: &Entry, check: &CheckButton) -> ListBoxRow {
	let row_box = gtk::Box::new(Orientation::Horizontal, 30);

	check.set_active(true);
	row_box.add(entry);
	row_box.add(check);

	let list_item = ListBoxRow::new();
	list_item.add(&row_box);

	list_item
}

pub fn build_ui(application: &gtk::Application) {
	let arduino_socket = Arc::new(Mutex::new(SocketAddr::from_str("0.0.0.0:0").unwrap()));

	let glade_src = include_str!("app_window.glade");
	let builder = Builder::new_from_string(glade_src);

	let window: ApplicationWindow = builder.get_object("connection_window").expect("Couldn't get connection_window");
	window.set_application(Some(application));

	let check_connection_button: Arc<Button> = Arc::new(builder.get_object("check_connection").expect("Couldn't get check_connetion_button"));
	let customize_button: Arc<Button> = Arc::new(builder.get_object("customize_button").expect("Couldn't get customize_button"));
	let ip_port_entry: Entry = builder.get_object("ip_address_entry").expect("Couldn't get ip_address_port_entry");

	window.show_all();

	{
		let customize_button = customize_button.clone();
		ip_port_entry.connect_changed(move |_| {
			customize_button.set_sensitive(false);
		});
	}

	{
		let customize_button = customize_button.clone();
		let arduino_socket = arduino_socket.clone();

		check_connection_button.connect_clicked(move |_| {
			let socket = match SocketAddr::from_str(&ip_port_entry.get_text().unwrap()) {
				Ok(s) => s,
				Err(_) => {
					message_box(DialogFlags::DESTROY_WITH_PARENT,
						MessageType::Error,
						ButtonsType::Ok,
						"Wrong ip:port pattern");
					return ();
				},
			};

			match network::check_connection(socket) {
				Ok(_) => {
					let mut arduino_socket = arduino_socket.lock().unwrap();
					arduino_socket.set_ip(socket.ip());
					arduino_socket.set_port(socket.port());

					message_box(DialogFlags::DESTROY_WITH_PARENT,
					MessageType::Info,
					ButtonsType::Ok, 
					"Connection established");
					customize_button.set_sensitive(true);
				},
				Err(e) => message_box(DialogFlags::DESTROY_WITH_PARENT, 
					MessageType::Warning,
					ButtonsType::Ok, 
					e.as_ref()),
			}
		});
	}

	{
		let app = Arc::new(application.clone());
		let builder = Arc::new(builder);
		let arduino_socket = arduino_socket.clone();

		customize_button.connect_clicked(move |_| {
			let arduino_socket = arduino_socket.lock().unwrap();

			build_customize_ui(app.clone(), builder.clone(), *arduino_socket);

			window.close();
		});
	}
}

fn build_customize_ui(application: Arc<gtk::Application>, builder: Arc<Builder> , arduino_socket: SocketAddr) {

	let window: ApplicationWindow = builder.get_object("modules_window").expect("Couldn't get modules_window");
	window.set_application(Some(&*application));

	let module_list_box: ListBox = builder.get_object("module_list_box").expect("Couldn't get module_list_box");
	let accept_button: Button = builder.get_object("accept_button").expect("Couldn't get accept_button");


	let mods: Vec<Box<dyn Module>> = Vec::new();

	let mods = Arc::new(Mutex::new(mods));
	let mut entries: Vec<Entry> = Vec::new(); 
	let mut check_buttons: Vec<CheckButton> = Vec::new(); 
	
	{	
		let mut com = mods.lock().unwrap();

			match network::get_module_list(arduino_socket) {
				Ok(modules) => {
					let mut id = 0;
					for module in modules.iter().skip(1) {
						match module.as_str() {
							"CZT" => com.push(TemperatureSensor::new_box(id)),
							"CZO" => com.push(DistanceSensor::new_box(id)),
							"CZW" => com.push(HumiditySensor::new_box(id)),
							"LAM" => com.push(LightSensor::new_box(id)),
							"BUZ" => com.push(Buzzer::new_box(id)),
							"RGB" => com.push(RGBLed::new_box(id)),
							_ => (),
						}
						id += 1;
					}
				}
				Err(_) => (),
			}
		

		for m in com.iter() {
			let check: CheckButton = CheckButton::new();
			let entry: Entry = Entry::new();
			entry.set_text(m.get_name());

			module_list_box.add(&list_row(&entry, &check));

			check_buttons.push(check);
			entries.push(entry);
		}
	}

	window.show_all();

	{
		let builder = builder.clone();

		accept_button.connect_clicked(move |_| {
			{	
				let mut com = mods.lock().unwrap();
				for i in 0..com.len() {
					if check_buttons[i].get_active() {
						com[i].set_active(true);
						com[i].set_name(&entries[i].get_text().unwrap());
					}
				}
			}

			build_control_ui(application.clone(), builder.clone(), arduino_socket, mods.clone());

			window.close();
		});
	}
}

fn build_control_ui(application: Arc<gtk::Application>, builder: Arc<Builder> , arduino_socket: SocketAddr, modules: Arc<Mutex<Vec<Box<dyn Module>>>>) {
	let window: ApplicationWindow = builder.get_object("control_window").expect("Couldn't get control_window");
	window.set_application(Some(&*application));

	let notebook: Notebook = builder.get_object("modules_notebook").expect("Couldn't get modules_notebook");
	let socket = Arc::new(arduino_socket);
	let mods = modules.lock().unwrap();

	for i in 0..mods.len() {
		notebook.append_page(&mods[i].get_container(socket.clone()), Some(&Label::new(Some(format!("#{}", i).as_str()))));
	}

	window.show_all();
}