fn main() {
	let reader = std::io::stdin();
	let mut lines : Vec<String> = Vec::new();
	let mut bytes : Vec<&u8> = Vec::new();
	reader.read_to_end(bytes);
	match reader.read_to_end(bytes) {
		Err(e) => println!("{}", e),
		_ => {}
	}

	// assume utf8
	for bytes_line in bytes.iter() {
		match std::string::String::from_utf8(bytes_line) {
			Ok(s) => {
				lines.push(s)
			},
			_ => {

			}
		}
	}

	for i in lines.iter() {
		println!("{}", i);
	}


}