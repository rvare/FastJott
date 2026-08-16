/*
Copyright (c) 2026 Richard Varela.
This file is part of FastJott, which is released under GPL v3.

FastJott is free software: you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software Foundation,
either version 3 of the License, or (at your option) any later version.

FastJott is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with FastJott.
If not, see <https://www.gnu.org/licenses/>.
*/

//! This crate contains all essential functions that perform the basic operations related a `notes.txt` file.

use chrono;
use std::io::Write;
use std::{env, fs, io, process};

/// Prompts user to write a new note, save it to the notes.txt file, with current date and optional important signifier.
///
/// The `important_flag` parameter is set by `-i`/`--important`.
/// When given, the parameter is set to `true` and a `(*)` is appended before the date.
/// If not given, then the parameter will be `false` and the signified will not be appended.
///
/// Four possible errors may happen:
/// 1. The program does not have access to the user's home directory.
/// 2. Could not open `notes.txt`.
/// 3. An I/O error occurs when hitting Enter after writing a new note.
/// 4. Could not write to `notes.txt`.
pub fn new_note(important_flag: bool) {
    // Returns a PathBuf and moves it to file_path.
    let Some(mut file_path) = env::home_dir() else {
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

/// Archives current notes.txt file into an archive directory in the same location.
///
/// The function will copy the user's notes.txt file into a directory called `NotesTxtArchive`.
/// The copied file will be renamed to `archive_<current date>.txt`.
///
/// If the directory does not exist, an error is thrown and the operation exits.
///
/// There are three possible errors:
/// 1. The program does not have access to the user's home directory.
/// 2. The program could not open `notes.txt`.
/// 3. Could not copy the file to the archive directory.
pub fn archive_notes_file() {
    // Returns a PathBuf and moves it to file_path.
    let Some(mut notes_path) = env::home_dir() else {
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

/// Given a query, will search the user's notes file for specific substring.
///
/// There are two possible errors:
/// 1. The program does not have access to the user's home directory.
/// 2. Could not open the user's `notes.txt` file.
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

/// Display help information.
pub fn help_info() {
    println!("Usage: FastJott [OPTION]");
    println!("-i, --important\n\tlabels a note as important");
    println!("-a, --archive\n\tarchives current notes.txt file");
    println!("-h, --help\n\tdisplay this help information");
    println!("-s, --search\n\tsearch for notes with specific query");
}
