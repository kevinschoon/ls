mod color;
mod file;
mod helper;
mod parser;

pub use crate::color::{paint, Color};
pub use crate::file::{get_files, resolve_group, resolve_user, to_unix_permission, File, Kind};
pub use crate::helper::pad_strings;
pub use crate::parser::parse;

use std::env::args;
use std::process::exit;
use std::time::UNIX_EPOCH;

fn to_file_name(file: &File) -> String {
    match file.kind {
        Kind::File => paint(file.name.clone(), Color::Normal),
        Kind::Dir => paint(file.name.clone(), Color::BlueFgBold),
        Kind::Link => paint(file.name.clone(), Color::CyanFgBold),
        Kind::Char => paint(file.name.clone(), Color::YellowFgBold),
        Kind::Socket => paint(file.name.clone(), Color::PurpleFg),
        _ => paint(file.name.clone(), Color::Normal),
    }
}

fn display_short(files: Vec<File>) {
    for file in files {
        print!("{}  ", to_file_name(&file))
    }
    println!(" ")
}

fn display_long(files: Vec<File>) {
    let rows: Vec<Vec<String>> = files
        .iter()
        .map(|f| {
            [
                to_unix_permission(f.mode),
                resolve_user(f.uid),
                resolve_group(f.gid),
                String::from(format!("{}", f.size).as_str()),
                f.modified
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
                to_file_name(f),
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
