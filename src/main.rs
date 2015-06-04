use std::io;
use std::io::Read;
extern crate libc;

fn main() {

	let istty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;
	if istty { // If there's no pipe
		println!("No piped input!");
	} else { // If there's piped input
		let stdin = io::stdin();
		let mut locked = stdin.lock();
		let mut all_lines : String = String::new();
		match locked.read_to_string(&mut all_lines) {
			Ok(a) => println!("stdin has length: {}", a),
			_ => println!("Error!")
		};

		let lines : Vec<&str> = all_lines.split("\n").collect();

		let mut curr_index : u32 = 0;
		let num_lines = lines.len() as u32;
		for line in lines {
			curr_index = curr_index + 1;
			print!("{}", line);
			if curr_index != num_lines {
				print!("\n");
			}
		}
	}
}

fn get_filenames(lines: Vec<&str>) -> Vec<&str> {
	
}