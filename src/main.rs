use std::{env, fs, io};
use std::io::Write;
use chrono;

fn new_note(important_flag: bool) {
	let Some(mut file_path) = env::home_dir() else { // Returns a BufPath and moves it to file_path.
		panic!("Could not get path to home directory!");
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

	let signifier: &str = if important_flag { "(*) " } else { "" };

	if let Err(why) = writeln!(file, "{}{}  {}", signifier, current_date.format("%Y-%m-%d"), note_content.trim()) {
		panic!("Could not write to file: {}", why);
	}

	println!("\nNote saved.");
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut important_flag: bool = false;

	if args.len() > 1 {
		let Some(flags) = args.get(1) else {
			panic!("Could not get arguments passed.");
		};

		if flags.chars().nth(0).unwrap() != '-' {
			panic!("Flags were not given! If you did give flags, make sure there is a dash next the flags.");
		}

		for flag in flags.chars() {
			if flag == 'i' {
				important_flag = true;
			}
		}
	}

	new_note(important_flag);
}
