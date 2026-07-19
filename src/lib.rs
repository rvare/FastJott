use std::{env, fs, io, process};
use std::io::Write;
use chrono;

pub fn new_note(important_flag: bool) {
	let Some(mut file_path) = env::home_dir() else { // Returns a BufPath and moves it to file_path.
		eprintln!("Could not get path to home directory!");
		process::exit(1);
	};

	file_path.push("rtest.txt");

	let mut file: fs::File = match fs::OpenOptions::new().append(true).create(true).open(file_path) {
		Ok(file) => file,
		Err(why) => {
			eprintln!("Could not open notes.txt file: {}", why);
			process::exit(1);
		},
	};

	println!("Start typing your note below. Hit ENTER once you are done.");
	let mut note_content = String::new();
	if let Err(why) = io::stdin().read_line(&mut note_content) {
		eprintln!("Input error: {}", why);
		process::exit(1);
	}

	let current_date: chrono::DateTime<chrono::Local> = chrono::Local::now();

	let signifier: &str = if important_flag { "(*) " } else { "" };

	if let Err(why) = writeln!(file, "{}{}  {}", signifier, current_date.format("%Y-%m-%d"), note_content.trim()) {
		eprintln!("Could not write to file: {}", why);
		process::exit(1);
	}

	println!("\nNote saved.");
}
