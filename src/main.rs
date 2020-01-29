use std::env::args;
use std::fs::{read_dir, DirEntry};
use std::io::{self, Write};
use std::process::exit;
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
    modified: SystemTime,
}

#[derive(Debug)]
struct CommandOptions {
    path: String,
    show_long: bool,
    show_all: bool,
}

fn show_usage() {
    let message = b"

Usage: ls [OPTION]... [PATH]

A pointless implementation of the Unix ls command to help me learn how to write programs in Rust.

Arguments:
-a --all        do not ignore entries starting with .
-l --long       use long listing format
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
    File {
        name: entry.file_name().into_string().unwrap(),
        kind: ftype,
        size: 0,
        modified: SystemTime::now(),
    }
}

fn get_files(path: String) -> Result<Vec<File>, std::io::Error> {
    let entries = read_dir(path);
    match entries {
        Ok(entries) => {
            let mut files: Vec<File> = Vec::new();
            for entry in entries {
                files.push(to_file(entry.unwrap()));
            }
            Ok(files)
        }
        Err(err) => Err(err),
    }
}

fn parse_args(args: &mut Vec<String>) -> Result<CommandOptions, String> {
    let mut opts = CommandOptions {
        path: String::from("."),
        show_long: false,
        show_all: false,
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
            _ => return Err(String::from(next.as_str())),
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
        if long {
            let mut display_value = String::new();
            // TODO I do not know how to properly format unix permissions
            match file.kind {
                Kind::File => display_value.push_str("-F-"),
                Kind::Dir => display_value.push_str("-D-"),
                Kind::Link => display_value.push_str("-L-"),
            }
            let unix = file
                .modified
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();
            stdout.write_all(display_value.as_bytes()).unwrap();
            stdout.write_all(b" ").unwrap();
            stdout.write_all(unix.as_bytes()).unwrap();
            stdout.write_all(b" ").unwrap();
            stdout.write_all(file.name.as_bytes()).unwrap();
            stdout.write_all(b"\n").unwrap()
        } else {
            stdout.write_all(file.name.as_bytes()).unwrap();
            stdout.write_all(b" ").unwrap();
        }
        if index == files.len() - 1 && !long {
            stdout.write_all(b"\n").unwrap()
        }
    }
}

fn main() {
    let mut args: Vec<String> = args().collect();
    let options = parse_args(&mut args);
    match options {
        Ok(opts) => {
            let files = get_files(opts.path);
            match files {
                Ok(files) => {
                    display_files(files, opts.show_long, opts.show_all);
                }
                Err(err) => {
                    println!("{}", err);
                    exit(1);
                }
            }
        }
        Err(err) => {
            println!("invalid command line option: ");
            println!("{}", err);
            show_usage();
            exit(1);
        }
    }
}
