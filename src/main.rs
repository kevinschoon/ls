mod file;
mod helper;
mod parser;

pub use crate::file::{get_files, File};
pub use crate::helper::pad_strings;
pub use crate::parser::parse;

use std::env::args;
use std::process::exit;
use std::time::UNIX_EPOCH;

fn display_short(files: Vec<File>) {
    for file in files {
        print!("{}  ", file.name)
    }
    println!(" ")
}

fn display_long(files: Vec<File>) {
    let rows: Vec<Vec<String>> = files
        .iter()
        .map(|f| {
            [
                String::from(format!("{}", f.mode & 0o7777).as_str()),
                String::from(format!("{}", f.uid).as_str()),
                String::from(format!("{}", f.gid).as_str()),
                String::from(format!("{}", f.size).as_str()),
                f.modified
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
                f.name.clone(),
            ]
            .to_vec()
        })
        .collect();
    let rows = pad_strings(rows);
    for row in rows {
        for col in row {
            print!("{} ", col)
        }
        println!(" ")
    }
}

fn main() {
    let mut args: Vec<String> = args().collect();
    let opts = parse(&mut args);
    let files = match get_files(opts.path, opts.show_all, opts.sort_lex) {
        Ok(files) => files,
        Err(err) => {
            println!("{}\n", err);
            exit(1);
        }
    };
    if opts.show_long {
        display_long(files)
    } else {
        display_short(files)
    }
}
