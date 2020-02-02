mod error;
mod helper;

pub use crate::error::{LSError, LSErrorKind};
pub use crate::helper::pad_strings;

use std::env::args;
use std::fs::{read_dir, DirEntry};
use std::io::{self, Write};
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

#[derive(Debug)]
struct CommandOptions {
    path: String,
    show_long: bool,
    show_all: bool,
    show_help: bool,
}

fn show_usage() {
    let message = b"
Usage: ls [OPTION]... [PATH]

A pointless implementation of the Unix ls command to help me learn how to write programs in Rust.

Arguments:
-a --all        do not ignore entries starting with .
-l --long       use long listing format
-h --help       display this help dialog
";
    io::stdout().write_all(message).unwrap()
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

fn get_files(path: String, show_all: bool) -> Result<Vec<File>, LSError> {
    let entries = read_dir(path.clone());
    match entries {
        Ok(entries) => {
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
        Err(_) => Err(LSError {
            message: String::from(path.as_str()),
            kind: LSErrorKind::PermissionDenied,
        }),
    }
}

fn parse_args(args: &mut Vec<String>) -> Result<CommandOptions, LSError> {
    let mut opts = CommandOptions {
        path: String::from("."),
        show_long: false,
        show_all: false,
        show_help: false,
    };
    // command name
    args.remove(0);
    // process remaining flags
    while let Some(next) = args.pop() {
        match next.as_str() {
            "-l" => opts.show_long = true,
            "--long" => opts.show_long = true,
            "-a" => opts.show_all = true,
            "--all" => opts.show_all = true,
            "-h" => opts.show_help = true,
            "--help" => opts.show_help = true,
            value => {
                return Err(LSError {
                    kind: LSErrorKind::InvalidArguments,
                    message: String::from(value),
                })
            }
        }
    }
    Ok(opts)
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
    let options = match parse_args(&mut args) {
        Ok(opts) => opts,
        Err(err) => {
            println!("invalid command line option: {}\n", err);
            show_usage();
            exit(1);
        }
    };
    if options.show_help {
        show_usage();
        exit(0);
    }
    let files = match get_files(options.path, options.show_all) {
        Ok(files) => files,
        Err(err) => {
            println!("{}\n", err);
            exit(1);
        }
    };
    if options.show_long {
        display_long(&files)
    } else {
        display_short(&files)
    }
}
