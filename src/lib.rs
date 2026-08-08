use chrono;
use std::io::Write;
use std::{env, fs, io, process};

pub fn new_note(important_flag: bool) {
    let Some(mut file_path) = env::home_dir() else { // Returns a PathBuf and moves it to file_path.
        eprintln!("Could not get path to home directory!");
        process::exit(1);
    };

    file_path.push("rtest.txt");

    let mut file: fs::File = match fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(why) => {
            eprintln!("Could not open notes.txt file: {}", why);
            process::exit(1);
        }
    };

    println!("Start typing your note below. Hit ENTER once you are done.");
    let mut note_content = String::new();
    if let Err(why) = io::stdin().read_line(&mut note_content) {
        eprintln!("Input error: {}", why);
        process::exit(1);
    }

    let current_date: chrono::DateTime<chrono::Local> = chrono::Local::now();

    let signifier: &str = if important_flag { "(*) " } else { "" };

    if let Err(why) = writeln!(
        file,
        "{}{}  {}",
        signifier,
        current_date.format("%Y-%m-%d"),
        note_content.trim()
    ) {
        eprintln!("Could not write to file: {}", why);
        process::exit(1);
    }

    println!("\nNote saved.");
}

pub fn archive_notes_file() {
    let Some(mut notes_path) = env::home_dir() else { // Returns a PathBuf and moves it to file_path.
        eprintln!("Could not get path to home directory!");
        process::exit(1);
    };

    let mut archive_path = notes_path.clone();

    notes_path.push("rtest.txt");

    archive_path.push("NotestxtBackUp");
    let current_date: chrono::DateTime<chrono::Local> = chrono::Local::now();
    archive_path.push(format!("archive_{}.txt", current_date.format("%Y%m%d")));

    if let Err(why) = fs::copy(&notes_path, &archive_path) {
        eprintln!("Input error: {}", why);
        process::exit(1);
    };

    println!("Archived to: {}", archive_path.display());
}

pub fn search_for_note(query: String) {
    let Some(mut file_path) = env::home_dir() else {
        eprintln!("Could not get to home directory!");
        process::exit(1);
    };

    file_path.push("rtest.txt");

    let note_contents: String = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(why) => {
            eprintln!("Could not open notes.txt file: {}", why);
            process::exit(1);
        }
    };

    let mut count = 1;
    for line in note_contents.lines() {
        if line.contains(&query) {
            println!("{count}: {}", line);
        }
	count += 1;
    }
}
