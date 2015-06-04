use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let mut locked = stdin.lock();

	let mut all_lines : String = String::new();

	println!("a");
	let stdin_bytes_count = locked.bytes().count();
	println!("{}", stdin_bytes_count);

	match locked.read_to_string(&mut all_lines) {
		Ok(a) => println!("stdin has length: {}", a),
		_ => println!("Error!")
	};
	println!("b");

	let lines : Vec<&str> = all_lines.split("\n").collect();

	for line in lines {
		println!("{}", line);
	}
}