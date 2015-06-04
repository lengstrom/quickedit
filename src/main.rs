use std::io;
use std::io::prelude::*;

fn main() {
	let lockedStdin = io::stdin().lock();
	let byteLines : Vec<u8> = Vec::new();
	let lines : Vec<String> = Vec::new();

	match lockedStdin.read_to_end(byteLines) {
		Ok(a) => println!("stdin has length: {}", a),
		_ => println!("Error!")
	}

	for byteLine in byteLines.iter() {
		lines.push(String::from_utf8(byteLine).unwrap());
	}

	for line in lines.iter() {
		println!("{}", line);
	}
}