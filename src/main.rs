use std::{env, fs, io};
use std::io::Write;

fn prompt_new_note() {
	let Some(mut file_path) = env::home_dir() else {
		panic!("Could not get your path!");
	};

	file_path.push("rtest.txt");

	let mut file = match fs::OpenOptions::new().append(true).create(true).open(file_path) {
		Ok(file) => file,
		Err(why) => panic!("Could not open notes.txt file: {}", why),
	};

	println!("Start typing your note below. Hit ENTER once you are done.");
	let mut note_content = String::new();
	if let Err(why) = io::stdin().read_line(&mut note_content) {
		panic!("Input error: {}", why);
	}

	if let Err(why) = writeln!(file, "{}", note_content.trim()) {
		panic!("Could not write to file: {}", why);
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		1 => {
			prompt_new_note();
		},
		// TODO Add more branches.
		_ => println!("Error"),
	}
}
