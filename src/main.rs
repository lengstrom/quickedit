fn main() {
	let reader = std::io::stdin();
	let lineReader = reader.lines();

	let mut lines : Vec<&str> = vec![];
	for line in lineReader.iter() {
		match lineContainer {
			Some(Ok(a)) => {
				lines.push(line);
			},
			_ => {

			}
		}
	}

	for i in lines.iter() {
		println!("{}", i);
	}


}