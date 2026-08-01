use std::env;

fn main() {
    let mut args_iter = env::args().skip(1);

    if let Some(flag) = args_iter.next() {
        match flag.as_str() {
            "-i" => rustnoteutil::new_note(true),
            "-a" => rustnoteutil::archive_notes_file(),
            _ => {
                eprintln!("Unrecognized flag");
                return;
            }
        }
    } else {
        rustnoteutil::new_note(false);
    }
}
