mod error;
mod helper;

pub use crate::error::{LSError, LSErrorKind};
pub use crate::helper::pad_strings;

use std::env::args;
use std::fs::{read_dir, DirEntry};
use std::io::{self, Write};
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::process::exit;
use std::string::String;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum Kind {
    File,
    Dir,
    Link,
    /*
    TODO
    Char,
    Block,
    Socket,
    Pipe,
    */
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

fn to_file(entry: DirEntry) -> File {
    let ft = entry.file_type().unwrap();
    let ftype = {
        if ft.is_dir() {
            Kind::Dir
        } else if ft.is_file() {
            Kind::File
        } else if ft.is_symlink() {
            Kind::Link
        } else {
            // Non-Unix platform??
            Kind::File
        }
    };
    let md = entry.metadata().unwrap();
    let perms = md.permissions();
    File {
        name: entry.file_name().into_string().unwrap(),
        kind: ftype,
        size: md.len(),
        uid: md.st_uid(),
        gid: md.st_gid(),
        mode: perms.mode(),
        modified: SystemTime::now(),
    }
}

fn get_files(path: String) -> Result<Vec<File>, LSError> {
    let entries = read_dir(path.clone());
    match entries {
        Ok(entries) => {
            let mut files: Vec<File> = Vec::new();
            entries.for_each(|entry| files.push(to_file(entry.unwrap())));

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
    let files = match get_files(options.path) {
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
