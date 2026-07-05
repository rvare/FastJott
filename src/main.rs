use std::{env, fs, io};
use std::io::Write;
use chrono;

fn new_note() {
	let Some(mut file_path) = env::home_dir() else { // Returns a BufPath and moves it to file_path.
		panic!("Could not get your path!");
	};

	file_path.push("rtest.txt");

	let mut file: fs::File = match fs::OpenOptions::new().append(true).create(true).open(file_path) {
		Ok(file) => file,
		Err(why) => panic!("Could not open notes.txt file: {}", why),
	};

	println!("Start typing your note below. Hit ENTER once you are done.");
	let mut note_content = String::new();
	if let Err(why) = io::stdin().read_line(&mut note_content) {
		panic!("Input error: {}", why);
	}

	let current_date: chrono::DateTime<chrono::Local> = chrono::Local::now();

	if let Err(why) = writeln!(file, "{}  {}", current_date.format("%Y-%m-%d"), note_content.trim()) {
		panic!("Could not write to file: {}", why);
	}

	println!("\nNote saved.");
}

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		1 => {
			new_note();
		},
		// TODO Add more branches.
		_ => println!("Error"),
	}
}
