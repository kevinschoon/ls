mod error;

pub use crate::error::{LSError, LSErrorKind};

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

fn display_files(files: Vec<File>, long: bool, all: bool) {
    let mut stdout = io::stdout();
    for (index, file) in files.iter().enumerate() {
        if file.name.starts_with('.') && !all {
            continue;
        }
        let mut display_value = String::new();
        if long {
            // TODO I do not know how to properly format unix permissions
            match file.kind {
                Kind::File => {}
                Kind::Dir => {}
                Kind::Link => {}
            }
            display_value.push_str(format!("{}", file.mode).as_str());
            // TODO user/group
            display_value.push_str(format!(" {} {} ", file.uid, file.gid).as_str());
            display_value.push_str(format!("{}", file.size).as_str());
            let unix_time = file
                .modified
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();
            let unix_time = unix_time.as_str();
            display_value.push_str(" ");
            display_value.push_str(unix_time);
            display_value.push_str(" ");
            display_value.push_str(file.name.as_str());
            display_value.push_str("\n");
        } else {
            display_value.push_str(file.name.as_str());
            display_value.push_str(" ");
        }
        if index == files.len() - 1 && !long {
            display_value.push_str("\n");
        }
        stdout.write_all(display_value.as_bytes()).unwrap();
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
    display_files(files, options.show_long, options.show_all);
}
