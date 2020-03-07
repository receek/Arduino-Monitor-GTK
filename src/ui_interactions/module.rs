extern crate gtk;

use gtk::prelude::*;
use gtk::{Adjustment, Button, Fixed, Label, Orientation, Scale, SpinButton};

use std::net::{SocketAddr};
use std::sync::{Arc, Mutex};

use crate::ui_interactions::network::*;

pub trait Module {
	fn set_active(&mut self, val: bool);
	fn set_id(&mut self, id: u32);
	fn get_name(&self) -> &str;
	fn set_name(&mut self, name: &str);
	
	fn get_container(&self, socket: Arc<SocketAddr>) -> Fixed;
}

pub struct TemperatureSensor {
	id: u32,
	name: String,
	active: bool,
}

impl TemperatureSensor {
	#[allow(dead_code)]
	pub fn new() -> Self {
		TemperatureSensor{
			id: 0,
			name: String::from("Temperature sensor"),
			active: false,
		}
	}

	pub fn new_box(id: u32) -> Box<dyn Module> {
		Box::new(
			TemperatureSensor{
				id: id,
				name: String::from("Temperature sensor"),
				active: false,
			}
		)
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

		let name_label: Label = Label::new(Some(self.name.as_str()));
		name_label.set_size_request(140, 40); 
		let temp_title_label: Label = Label::new(Some("Temperature:"));
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
						temp_label.set_label(format!("{}°C",vec[0]).as_str());
					}
					Err(e) => temp_label.set_label(e),
				}
			});
		}

		let container: Fixed = Fixed::new(); 
		container.set_size_request(240, 160);
		container.put(&name_label, 20, 20);
		container.put(&temp_title_label, 20, 60);
		container.put(&*temp_label, 120, 60);
		container.put(&request_button, 20, 100);

		container
	}
}

pub struct DistanceSensor {
	id: u32,
	name: String,
	active: bool,
}

impl DistanceSensor {
	#[allow(dead_code)]
	pub fn new() -> Self {
		DistanceSensor{
			id: 0,
			name: String::from("Distance sensor"),
			active: false,
		}
	}

	pub fn new_box(id: u32) -> Box<dyn Module> {
		Box::new(
			DistanceSensor{
				id: id,
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

		let name_label: Label = Label::new(Some(self.name.as_str()));
		name_label.set_size_request(140, 40); 
		let dist_title_label: Label = Label::new(Some("Distance:"));
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
						dist_label.set_label(format!("{} cm",vec[0]).as_str());
					}
					Err(e) => dist_label.set_label(e),
				}
			});
		}

		let container: Fixed = Fixed::new(); 
		container.set_size_request(240, 160);
		container.put(&name_label, 20, 20);
		container.put(&dist_title_label, 20, 60);
		container.put(&dist_label, 120, 60);
		container.put(&request_button, 20, 100);

		container
	}
}


pub struct HumiditySensor {
	id: u32,
	name: String,
	active: bool,
}

impl HumiditySensor {
	#[allow(dead_code)]
	pub fn new() -> Self {
		HumiditySensor{
			id: 0,
			name: String::from("Humidity sensor"),
			active: false,
		}
	}

	pub fn new_box(id: u32) -> Box<dyn Module> {
		Box::new(
			HumiditySensor{
				id: id,
				name: String::from("Humidity sensor"),
				active: false,
			}
		)
	}
}

impl Module for HumiditySensor {
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
		let hum_label: Label = Label::new(Some("-"));
		hum_label.set_size_request(100, 40);

		let name_label: Label = Label::new(Some(self.name.as_str()));
		name_label.set_size_request(140, 40); 
		let hum_title_label: Label = Label::new(Some("Humidity:"));
		hum_title_label.set_size_request(100, 40);

		let request_button: Button = Button::new();
		request_button.set_label("Request");
		request_button.set_size_request(60, 40);

		{
			let id = self.id.clone();
			let hum_label = hum_label.clone();
			request_button.connect_clicked(move |_| {
				match get_value(*socket, id) {
					Ok(response) => {
						let vec: Vec<&str> = response.split("\n").collect(); 
						hum_label.set_label(format!("{}%",vec[0]).as_str());
					}
					Err(e) => hum_label.set_label(e),
				}
			});
		}

		let container: Fixed = Fixed::new(); 
		container.set_size_request(240, 160);
		container.put(&name_label, 20, 20);
		container.put(&hum_title_label, 20, 60);
		container.put(&hum_label, 120, 60);
		container.put(&request_button, 20, 100);

		container
	}
}

pub struct LightSensor {
	id: u32,
	name: String,
	active: bool,
}

impl LightSensor {
	#[allow(dead_code)]
	pub fn new() -> Self {
		LightSensor{
			id: 0,
			name: String::from("Light sensor"),
			active: false,
		}
	}

	pub fn new_box(id: u32) -> Box<dyn Module> {
		Box::new(
			LightSensor{
				id: id,
				name: String::from("Light sensor"),
				active: false,
			}
		)
	}
}

impl Module for LightSensor {
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
		let enabled: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

		let light_label: Arc<Label> = Arc::new(Label::new(Some("-")));
		light_label.set_size_request(100, 40);

		let on_off_label: Label = Label::new(Some("-"));
		on_off_label.set_size_request(100, 40);

		let name_label: Label = Label::new(Some(self.name.as_str()));
		name_label.set_size_request(140, 40);

		let light_title_label: Label = Label::new(Some("Brightness:"));
		light_title_label.set_size_request(100, 40);

		let on_off_title_label: Label = Label::new(Some("Lamp is:"));
		on_off_title_label.set_size_request(100, 40);


		let request_button: Button = Button::new();
		request_button.set_label("Request");
		request_button.set_size_request(60, 40);

		let switch_button: Arc<Button> = Arc::new(Button::new());
		switch_button.set_label("Switch");
		switch_button.set_size_request(60, 40);
		switch_button.set_sensitive(false);

		{
			let id = self.id.clone();
			let light_label = light_label.clone();
			let on_off_label = on_off_label.clone();
			let enabled = enabled.clone();
			let socket = socket.clone();
			let switch_button = switch_button.clone();

			request_button.connect_clicked(move |_| {
				match get_value(*socket, id) {
					Ok(response) => {
						let mut state = enabled.lock().unwrap();
						let vec: Vec<&str> = response.split("\n").collect(); 
						let procc = vec[0].parse::<f64>().unwrap() / 1024.0 * 100.0;
						light_label.set_label(format!("{:.2}%", procc).as_str());
						on_off_label.set_label(format!("{}", vec[1]).as_str());
						match vec[1] {
							"ON" => *state = true,
							"OFF" => *state = false,
							_ => (),
						}
						switch_button.set_sensitive(true);
					}
					Err(e) => {
						light_label.set_label(e);
						switch_button.set_sensitive(false);
					}
				}
			});
		}

		{
			let id = self.id.clone();
			let on_off_label = on_off_label.clone();
			let light_label = light_label.clone();
			let enabled = enabled.clone();

			switch_button.connect_clicked(move |btn| {
				let mut state = enabled.lock().unwrap();
				let command = if *state {"STP"} else {"STR"};

				match (set_value(*socket, id, command), *state) {
					(Ok(_), true) => {
						on_off_label.set_label("OFF");
						*state = !*state;
					}
					(Ok(_), false) => {
						on_off_label.set_label("ON");
						*state = !*state;
					}
					(Err(e), _) => {
						light_label.set_label(e);
						btn.set_sensitive(false);
					}
				}
			});
		}

		let container: Fixed = Fixed::new(); 
		container.set_size_request(240, 200);
		container.put(&name_label, 20, 20);
		container.put(&light_title_label, 20, 60);
		container.put(&*light_label, 120, 60);
		container.put(&on_off_title_label, 20, 100);
		container.put(&on_off_label, 120, 100);
		container.put(&request_button, 20, 140);
		container.put(&*switch_button, 120, 140);

		container
	}
}

pub struct Buzzer {
	id: u32,
	name: String,
	active: bool,
}

impl Buzzer {
	#[allow(dead_code)]
	pub fn new() -> Self {
		Buzzer{
			id: 0,
			name: String::from("Buzzer"),
			active: false,
		}
	}

	pub fn new_box(id: u32) -> Box<dyn Module> {
		Box::new(
			Buzzer{
				id: id,
				name: String::from("Buzzer"),
				active: false,
			}
		)
	}
}

impl Module for Buzzer {
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
		let adjustment: Adjustment = Adjustment::new(0.0, 0.0, 255.0, 1.0, 10.0, 0.0);

		let buzzer_value_spin: Arc<SpinButton> = Arc::new(SpinButton::new(Some(&adjustment), 1.0, 0));
		buzzer_value_spin.set_size_request(100, 40);
		buzzer_value_spin.set_sensitive(false);

		let name_label: Label = Label::new(Some(self.name.as_str()));
		name_label.set_size_request(140, 40);

		let buzzer_title_label: Label = Label::new(Some("Value:"));
		buzzer_title_label.set_size_request(100, 40);

		let request_button: Button = Button::new();
		request_button.set_label("Request");
		request_button.set_size_request(100, 40);

		let change_value_button: Arc<Button> = Arc::new(Button::new());
		change_value_button.set_label("Change value");
		change_value_button.set_size_request(100, 40);
		change_value_button.set_sensitive(false);

		let sound_on_button: Arc<Button> = Arc::new(Button::new());
		sound_on_button.set_label("Make sound");
		sound_on_button.set_size_request(120, 40);
		sound_on_button.set_sensitive(false);

		let sound_off_button: Arc<Button> = Arc::new(Button::new());
		sound_off_button.set_label("Stop sound");
		sound_off_button.set_size_request(100, 40);
		sound_off_button.set_sensitive(false);

		{
			let id = self.id.clone();
			let socket = socket.clone();
			let buzzer_value_spin = buzzer_value_spin.clone();
			let change_value_button = change_value_button.clone();
			let sound_on_button = sound_on_button.clone();
			let sound_off_button = sound_off_button.clone();

			request_button.connect_clicked(move |_| {
				match get_value(*socket, id) {
					Ok(response) => {
						let val = response.trim();
						let mut access = true;
						match val {
							"BLC" => access = false,
							v => buzzer_value_spin.set_value(v.parse::<f64>().unwrap()),
						}
						buzzer_value_spin.set_sensitive(access);
						change_value_button.set_sensitive(access);
						sound_on_button.set_sensitive(access);
						sound_off_button.set_sensitive(access);
					}
					Err(_) => {
						buzzer_value_spin.set_value(0.0);
						buzzer_value_spin.set_sensitive(false);
						change_value_button.set_sensitive(false);
						sound_on_button.set_sensitive(false);
						sound_off_button.set_sensitive(false);
					}
				}
			});
		}

		{
			let id = self.id.clone();
			let socket = socket.clone();
			let buzzer_value_spin = buzzer_value_spin.clone();
			let sound_on_button = sound_on_button.clone();
			let sound_off_button = sound_off_button.clone();

			change_value_button.connect_clicked(move |btn| { 
				let command = format!("CHN {}", buzzer_value_spin.get_text().unwrap());

				match set_value(*socket, id, command.as_str()) {
					Ok(response) => {
						let val = response.trim();
						if val == "BLC" {
							buzzer_value_spin.set_value(0.0);
							buzzer_value_spin.set_sensitive(false);
							btn.set_sensitive(false);
							sound_on_button.set_sensitive(false);
							sound_off_button.set_sensitive(false);
						}
					}
					Err(_) => {
						buzzer_value_spin.set_value(0.0);
						buzzer_value_spin.set_sensitive(false);
						btn.set_sensitive(false);
						sound_on_button.set_sensitive(false);
						sound_off_button.set_sensitive(false);
					}
					
				}
			});
		}

		{
			let id = self.id.clone();
			let socket = socket.clone();
			let buzzer_value_spin = buzzer_value_spin.clone();
			let change_value_button = change_value_button.clone();
			let sound_off_button = sound_off_button.clone();

			sound_on_button.connect_clicked(move |btn| {
				let command = "STR";

				match set_value(*socket, id, command) {
					Ok(response) => {
						let val = response.trim();
						if val == "BLC" {
							buzzer_value_spin.set_value(0.0);
							buzzer_value_spin.set_sensitive(false);
							change_value_button.set_sensitive(false);
							btn.set_sensitive(false);
							sound_off_button.set_sensitive(false);
						}
					}
					Err(_) => {
						buzzer_value_spin.set_value(0.0);
						buzzer_value_spin.set_sensitive(false);
						change_value_button.set_sensitive(false);
						btn.set_sensitive(false);
						sound_off_button.set_sensitive(false);
					}
				}
			});
		}

		{
			let id = self.id.clone();
			let socket = socket.clone();
			let buzzer_value_spin = buzzer_value_spin.clone();
			let change_value_button = change_value_button.clone();
			let sound_on_button = sound_on_button.clone();


			sound_off_button.connect_clicked(move |btn| {
				let command = "STP";

				match set_value(*socket, id, command) {
					Ok(response) => {
						let val = response.trim();
						if val == "BLC" {
							buzzer_value_spin.set_value(0.0);
							buzzer_value_spin.set_sensitive(false);
							change_value_button.set_sensitive(false);
							sound_on_button.set_sensitive(false);
							btn.set_sensitive(false);
						}
					}
					Err(_) => {
						buzzer_value_spin.set_value(0.0);
						buzzer_value_spin.set_sensitive(false);
						change_value_button.set_sensitive(false);
						sound_on_button.set_sensitive(false);
						btn.set_sensitive(false);
					}
				}
			});
		}

		let container: Fixed = Fixed::new(); 
		container.set_size_request(240, 240);
		container.put(&name_label, 20, 20);
		container.put(&buzzer_title_label, 20, 60);
		container.put(&*buzzer_value_spin, 120, 60);
		container.put(&request_button, 20, 120);
		container.put(&*change_value_button, 160, 120);
		container.put(&*sound_on_button, 20, 180);
		container.put(&*sound_off_button, 160, 180);
		
		container
	}
}

pub struct RGBLed {
	id: u32,
	name: String,
	active: bool,
}

impl RGBLed {
	#[allow(dead_code)]
	pub fn new() -> Self {
		RGBLed{
			id: 0,
			name: String::from("RGB Led"),
			active: false,
		}
	}

	pub fn new_box(id: u32) -> Box<dyn Module> {
		Box::new(
			RGBLed{
				id: id,
				name: String::from("RGB Led"),
				active: false,
			}
		)
	}
}

impl Module for RGBLed {
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
		let enabled: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

		let name_label: Label = Label::new(Some(self.name.as_str()));
		name_label.set_size_request(100, 40); 

		let rgb_title_label: Label = Label::new(Some("RGB values:"));
		rgb_title_label.set_size_request(100, 40);

		let on_off_label: Arc<Label> = Arc::new(Label::new(Some("-")));
		on_off_label.set_size_request(40, 40);

		let on_off_title_label: Arc<Label> = Arc::new(Label::new(Some("Led is:")));
		on_off_title_label.set_size_request(100, 40);

		let adjustment1: Adjustment = Adjustment::new(0.0, 0.0, 255.0, 1.0, 10.0, 0.0);
		let adjustment2: Adjustment = Adjustment::new(0.0, 0.0, 255.0, 1.0, 10.0, 0.0);
		let adjustment3: Adjustment = Adjustment::new(0.0, 0.0, 255.0, 1.0, 10.0, 0.0);
		let r_scale: Arc<Scale> = Arc::new(Scale::new(Orientation::Vertical, Some(&adjustment1)));
		r_scale.set_size_request(50,150);
		r_scale.set_slider_size_fixed(true);
		r_scale.set_sensitive(false);
		r_scale.set_digits(0);
		let g_scale: Arc<Scale> = Arc::new(Scale::new(Orientation::Vertical, Some(&adjustment2)));
		g_scale.set_size_request(50,150);
		g_scale.set_slider_size_fixed(true);
		g_scale.set_sensitive(false);
		g_scale.set_digits(0);
		let b_scale: Arc<Scale> = Arc::new(Scale::new(Orientation::Vertical, Some(&adjustment3)));
		b_scale.set_size_request(50,150);
		b_scale.set_slider_size_fixed(true);
		b_scale.set_sensitive(false);
		b_scale.set_digits(0);

		let request_button: Button = Button::new();
		request_button.set_label("Request");
		request_button.set_size_request(60, 40);

		let change_values_button: Arc<Button> = Arc::new(Button::new());
		change_values_button.set_label("Change values");
		change_values_button.set_size_request(100, 40);

		let switch_button: Arc<Button> = Arc::new(Button::new());
		switch_button.set_label("Switch");
		switch_button.set_size_request(100, 40);

		{
			let id = self.id.clone();
			let socket = socket.clone();
			let r_scale = r_scale.clone();
			let g_scale = g_scale.clone();
			let b_scale = b_scale.clone();
			let change_values_button = change_values_button.clone();
			let on_off_label = on_off_label.clone();
			let switch_button = switch_button.clone();
			let enabled = enabled.clone();
			
			request_button.connect_clicked(move |_| {
				match get_value(*socket, id) {
					Ok(response) => {
						let vec: Vec<&str> = response.split("\n").collect(); 
						match vec[0] {
							"BLC" => {
								r_scale.set_sensitive(false);
								g_scale.set_sensitive(false);
								b_scale.set_sensitive(false);
								change_values_button.set_sensitive(false);
								switch_button.set_sensitive(false);
							},
							_ => {
								r_scale.set_value(vec[0].parse::<f64>().unwrap());
								r_scale.set_sensitive(true);
								g_scale.set_value(vec[1].parse::<f64>().unwrap());
								g_scale.set_sensitive(true);
								b_scale.set_value(vec[2].parse::<f64>().unwrap());
								b_scale.set_sensitive(true);
								change_values_button.set_sensitive(true);
								switch_button.set_sensitive(true);
								on_off_label.set_label(vec[3]);
								let mut enabled = enabled.lock().unwrap();
								*enabled = match vec[3] {"ON" => true, _ => false};
							},
						}
					}
					Err(_) => {
						r_scale.set_sensitive(false);
						g_scale.set_sensitive(false);
						b_scale.set_sensitive(false);
						change_values_button.set_sensitive(false);
						switch_button.set_sensitive(false);
					},
				}
			});
		}

		{
			let id = self.id.clone();
			let socket = socket.clone();
			let r_scale = r_scale.clone();
			let g_scale = g_scale.clone();
			let b_scale = b_scale.clone();
			let on_off_label = on_off_label.clone();
			let switch_button = switch_button.clone();
			
			change_values_button.connect_clicked(move |btn| {
				let command = format!("CHN {} {} {}", 
					r_scale.get_value(),
					g_scale.get_value(),
					b_scale.get_value());

				match set_value(*socket, id, command.as_str()) {
					Ok(response) => {
						let val = response.trim(); 
						match val {
							"BLC" => {
								r_scale.set_sensitive(false);
								g_scale.set_sensitive(false);
								b_scale.set_sensitive(false);
								btn.set_sensitive(false);
								switch_button.set_sensitive(false);
								on_off_label.set_label("-");
							},
							_ => (),
						}
					}
					Err(_) => {
						r_scale.set_sensitive(false);
						g_scale.set_sensitive(false);
						b_scale.set_sensitive(false);
						btn.set_sensitive(false);
						switch_button.set_sensitive(false);
						on_off_label.set_label("-");
					},
				}
			});
		}

		{
			let id = self.id.clone();
			let socket = socket.clone();
			let r_scale = r_scale.clone();
			let g_scale = g_scale.clone();
			let b_scale = b_scale.clone();
			let on_off_label = on_off_label.clone();
			let change_values_button = change_values_button.clone();
			
			switch_button.connect_clicked(move |btn| {
				let mut state = enabled.lock().unwrap();
				let command = if *state {"STP"} else {"STR"};

				match (set_value(*socket, id, command), *state) {
					(Ok(_), true) => {
						on_off_label.set_label("OFF");
						*state = !*state;
					}
					(Ok(_), false) => {
						on_off_label.set_label("ON");
						*state = !*state;
					}
					(Err(_), _) => ({
						r_scale.set_sensitive(false);
						g_scale.set_sensitive(false);
						b_scale.set_sensitive(false);
						change_values_button.set_sensitive(false);
						btn.set_sensitive(false);
						on_off_label.set_label("-");
					})
				}
			});
		}

		let container: Fixed = Fixed::new(); 
		container.set_size_request(240, 160);
		container.put(&name_label, 20, 20);
		container.put(&rgb_title_label, 20, 60);
		container.put(&*r_scale, 180, 10);
		container.put(&*g_scale, 240, 10);
		container.put(&*b_scale, 300, 10);
		container.put(&*on_off_title_label, 20, 100);
		container.put(&*on_off_label, 120, 100);
		container.put(&request_button, 20, 160);
		container.put(&*change_values_button, 120, 160);
		container.put(&*switch_button, 260, 160);

		container
	}
}