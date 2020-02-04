use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

pub fn check_connection(socket: SocketAddr) -> Result<(), &'static str> {

	if let Ok(mut stream) = TcpStream::connect_timeout(&socket, Duration::new(5,0)) {

		if let Err(_) = stream.set_read_timeout(Some(Duration::new(5,0))) {
			return Err("Cannot set timelimit on socket");
		}

		if let Err(_) = stream.write("HLL\n".as_bytes()) {
			return Err("Cannot send control packet, timelimit");
		}

		let mut buffer = String::new();
		match stream.read_to_string(&mut buffer) {
			Ok(_) => (),
			Err(_) => return Err("Response buffering error"),
		}

		match buffer.as_str() {
			"AVL\n" => Ok(()),
			_ => Err("It is not a Arduino"),
		}
	}
	else {
		Err("Cannot create stream, maybe wrong ip:port?")
	}
}

pub fn get_module_list(socket: SocketAddr) -> Result<Vec<String>, &'static str> {

	if let Ok(mut stream) = TcpStream::connect_timeout(&socket, Duration::new(5,0)) {

		if let Err(_) = stream.set_read_timeout(Some(Duration::new(5,0))) {
			return Err("Cannot set timelimit on socket");
		}

		if let Err(_) = stream.write("ASK\n".as_bytes()) {
			return Err("Cannot send control packet, timelimit");
		}

		let mut buffer = String::new();
		match stream.read_to_string(&mut buffer) {
			Ok(_) => (),
			Err(_) => return Err("Response buffering error"),
		}
		
		let result: Vec<String> = buffer.split("\n").map(|l| l.to_string()).collect();
		Ok(result)
	}
	else {
		Err("Arduino unreachable")
	}

}

pub fn get_value(socket: SocketAddr, index: u32) -> Result<String, &'static str> {
	if let Ok(mut stream) = TcpStream::connect_timeout(&socket, Duration::new(5,0)) {

		if let Err(_) = stream.set_read_timeout(Some(Duration::new(5,0))) {
			return Err("Cannot set timelimit on socket");
		}

		if let Err(_) = stream.write(format!("VAL {}\n", index).as_bytes()) {
			return Err("Cannot ask for value, timelimit");
		}

		let mut buffer = String::new();
		match stream.read_to_string(&mut buffer) {
			Ok(_) => (),
			Err(_) => return Err("Response buffering error"),
		}
		
		Ok(buffer)
	}
	else {
		Err("Arduino unreachable")
	}
}

pub fn set_value(socket: SocketAddr, index: u32, command: &str) -> Result<String, &'static str> {
	if let Ok(mut stream) = TcpStream::connect_timeout(&socket, Duration::new(5,0)) {

		if let Err(_) = stream.set_read_timeout(Some(Duration::new(5,0))) {
			return Err("Cannot set timelimit on socket");
		}

		if let Err(_) = stream.write(format!("COM {} {}\n", index, command).as_bytes()) {
			return Err("Cannot send command, timelimit");
		}

		let mut buffer = String::new();
		match stream.read_to_string(&mut buffer) {
			Ok(_) => (),
			Err(_) => return Err("Response buffering error"),
		}
		
		Ok(buffer)
	}
	else {
		Err("Arduino unreachable")
	}
}