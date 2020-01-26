extern crate gtk;

use gtk::prelude::*;

use gtk::{ApplicationWindow ,Builder, Button, CheckButton, Entry, Fixed, ButtonsType, DialogFlags, Orientation, Label, ListBox, ListBoxRow, MessageType, MessageDialog, Window};

use std::ops::{Index, IndexMut};

use crate::ui_interactions::network::*;

use std::str::FromStr;

use std::net::{SocketAddr};
use std::sync::{Arc, Mutex};

/* enum Data {
	Temp(u32),
	Light(u32),
} */


pub trait Module {
	fn set_active(&mut self, val: bool);
	fn set_id(&mut self, id: u32);
	fn get_name(&self) -> &str;
	fn set_name(&mut self, name: &str);
	
	fn get_container(&self, socket: Arc<SocketAddr>) -> Fixed;
}

pub struct Components {
	modules: Vec<Box<dyn Module>>,
}

impl Components {
	pub fn new() -> Self {
		Components { modules: Vec::new() }
	}
	
	pub fn add(&mut self, modu: Box<dyn Module>) {
		self.modules.push(modu);
	}

	pub fn len(&self) -> usize {
		self.modules.len()
	}
	
	pub fn get_modules(&self) -> &Vec<Box<dyn Module>> {
		self.modules.as_ref()
	}
}


impl Index<usize> for Components {
	type Output = Box<dyn Module>;

	fn index(&self, id: usize) -> &Self::Output {
		&self.modules[id]
	}
}

impl IndexMut<usize> for Components {
	fn index_mut(&mut self, id: usize) -> &mut Self::Output {
		&mut self.modules[id]
	}
}

pub struct TemperatureSensor {
	id: u32,
	value: i32,
	name: String,
	active: bool,
}

impl TemperatureSensor {
	pub fn new() -> Self {
		TemperatureSensor{
			id: 0,
			value: 0,
			name: String::from("Temperature sensor"),
			active: false,
		}
	}

	pub fn new_box(id: u32) -> Box<dyn Module> {
		Box::new(
			TemperatureSensor{
				id: id,
				value: 0,
				name: String::from("Temperature sensor"),
				active: false,
			}
		)
	}

	pub fn set_id(&mut self, id: u32) {
		self.id = id
	}
}

impl Module for TemperatureSensor {
	fn set_active(&mut self, val: bool) {
		self.active = val;
	}

	fn get_name(&self) -> &str {
		self.name.as_str()
	}

	fn set_name(&mut self, name: &str) {
		self.name = name.to_string();
	}

	fn set_id(&mut self, id: u32) {
		self.id = id
	}

	fn get_container(&self, socket: Arc<SocketAddr>) -> Fixed {
		let temp_label: Arc<Label> = Arc::new(Label::new(Some("-")));
		temp_label.set_size_request(100, 40);

		let temp_title_label: Label = Label::new(Some("Temperature"));
		temp_title_label.set_size_request(100, 40);

		let request_button: Button = Button::new();
		request_button.set_label("Request");
		request_button.set_size_request(60, 40);

		{
			let id = self.id.clone();
			let temp_label = temp_label.clone();
			request_button.connect_clicked(move |_| {
				match get_value(*socket, id) {
					Ok(response) => {
						let vec: Vec<&str> = response.split("\n").collect(); 
						temp_label.set_label(vec[0]);
					}
					_ => (),
				}
			});
		}

		let container: Fixed = Fixed::new(); 
		container.set_size_request(240, 120);
		container.put(&temp_title_label, 20, 20);
		container.put(&*temp_label, 120, 20);
		container.put(&request_button, 20, 60);

		container
	}
}

pub struct DistanceSensor {
	id: u32,
	value: u32,
	name: String,
	active: bool,
	//socket: SocketAddr,
}

impl DistanceSensor {
	pub fn new() -> Self {
		DistanceSensor{
			id: 0,
			value: 0,
			name: String::from("Distance sensor"),
			active: false,
		}
	}

	pub fn new_box(id: u32) -> Box<dyn Module> {
		Box::new(
			DistanceSensor{
				id: id,
				value: 0,
				name: String::from("Distance sensor"),
				active: false,
			}
		)
	}
}

impl Module for DistanceSensor {
	fn set_active(&mut self, val: bool) {
		self.active = val;
	}

	fn get_name(&self) -> &str {
		self.name.as_str()
	}

	fn set_name(&mut self, name: &str) {
		self.name = name.to_string();
	}

	fn set_id(&mut self, id: u32) {
		self.id = id
	}

	fn get_container(&self, socket: Arc<SocketAddr>) -> Fixed {
		let dist_label: Label = Label::new(Some("-"));
		dist_label.set_size_request(100, 40);

		let dist_title_label: Label = Label::new(Some("Distance"));
		dist_title_label.set_size_request(100, 40);

		let request_button: Button = Button::new();
		request_button.set_label("Request");
		request_button.set_size_request(60, 40);

		{
			let id = self.id.clone();
			let dist_label = dist_label.clone();
			request_button.connect_clicked(move |_| {
				match get_value(*socket, id) {
					Ok(response) => {
						let vec: Vec<&str> = response.split("\n").collect(); 
						dist_label.set_label(vec[0]);
					}
					_ => (),
				}
			});
		}

		let container: Fixed = Fixed::new(); 
		container.set_size_request(240, 120);
		container.put(&dist_title_label, 20, 20);
		container.put(&dist_label, 120, 20);
		container.put(&request_button, 20, 60);

		container
	}
}

/* 
struct Humidity_sensor {
	id: u32,
	value: f64,
	name: String,
} */