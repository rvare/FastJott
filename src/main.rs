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

use std::{env, process};

fn main() {
    let mut args_iter = env::args().skip(1);

    if let Some(flag) = args_iter.next() {
        match flag.as_str() {
            "-i" | "--important" => fastjott::new_note(true),
            "-a" | "--archive" => fastjott::archive_notes_file(),
            "-h" | "--help" => fastjott::help_info(),
            "-s" | "--serach" => {
                let Some(regex) = args_iter.next() else {
                    eprintln!("No search parameter provided");
                    process::exit(1);
                };
                fastjott::search_for_note(regex);
            }
            _ => {
                eprintln!("Unrecognized flag");
                return;
            }
        }
    } else {
        fastjott::new_note(false);
    }
}
