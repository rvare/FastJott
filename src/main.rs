use std::{env, process};

fn main() {
    let mut args_iter = env::args().skip(1);

    if let Some(flag) = args_iter.next() {
        match flag.as_str() {
            "-i" => rustnoteutil::new_note(true),
            "-a" => rustnoteutil::archive_notes_file(),
            "-s" => {
                let Some(regex) = args_iter.next() else {
                    eprintln!("No search parameter provided");
                    process::exit(1);
                };
                rustnoteutil::search_for_note(regex);
            }
            _ => {
                eprintln!("Unrecognized flag");
                return;
            }
        }
    } else {
        rustnoteutil::new_note(false);
    }
}
