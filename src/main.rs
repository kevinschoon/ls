mod helper;
mod parser;

pub use crate::helper::pad_strings;
pub use crate::parser::parse;

use std::env::args;
use std::fs::{read_dir, DirEntry};
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;
use std::process::exit;
use std::string::String;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum Kind {
    File,
    Dir,
    Link,
    Fifo,
    Block,
    Char,
    Socket,
}

#[derive(Debug)]
struct File {
    name: String,
    kind: Kind,
    size: u64,
    uid: u32,
    gid: u32,
    mode: u32,
    modified: SystemTime,
}

fn unknown_file() -> File {
    File {
        name: String::from("???"),
        kind: Kind::File,
        size: 0,
        uid: 0,
        gid: 0,
        mode: 0,
        modified: std::time::SystemTime::now(),
    }
}

fn to_file(entry: DirEntry) -> File {
    let ft = entry.file_type().unwrap();
    let ftype = {
        if ft.is_dir() {
            Kind::Dir
        } else if ft.is_file() {
            Kind::File
        } else if ft.is_symlink() {
            Kind::Link
        } else if ft.is_block_device() {
            Kind::Block
        } else if ft.is_char_device() {
            Kind::Char
        } else if ft.is_fifo() {
            Kind::Fifo
        } else if ft.is_socket() {
            Kind::Socket
        } else {
            // Non-Unix platform??
            panic!("unknown file type")
        }
    };
    let md = entry.metadata().unwrap();
    let perms = md.permissions();
    let file_name = entry.file_name().into_string().unwrap();
    File {
        name: file_name,
        kind: ftype,
        size: md.len(),
        uid: md.st_uid(),
        gid: md.st_gid(),
        mode: perms.mode(),
        modified: SystemTime::now(),
    }
}

fn get_files(path: String, show_all: bool) -> std::io::Result<Vec<File>> {
    let entries = read_dir(path)?;
    let files: Vec<File> = entries
        .map(|entry| match entry {
            Ok(entry) => to_file(entry),
            _ => unknown_file(),
        })
        .filter(|file| {
            if !show_all && file.name.starts_with('.') {
                return false;
            }
            true
        })
        .collect();
    Ok(files)
}

fn display_short(files: &[File]) {
    for file in files {
        print!("{} ", file.name)
    }
    println!(" ")
}

fn display_long(files: &[File]) {
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
    let files = match get_files(opts.path, opts.show_all) {
        Ok(files) => files,
        Err(err) => {
            println!("{}\n", err);
            exit(1);
        }
    };
    if opts.show_long {
        display_long(&files)
    } else {
        display_short(&files)
    }
}
