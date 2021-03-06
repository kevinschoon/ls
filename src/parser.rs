use std::process::exit;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParserError {
    message: String,
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bad argument: {}", self.message)
    }
}

pub struct CommandOptions {
    pub path: String,
    pub show_long: bool,
    pub show_all: bool,
    pub sort_lex: bool,
    pub show_help: bool,
}

fn show_usage() {
    let message = "
Usage: ls [OPTION]... [PATH]

A pointless implementation of the Unix ls command to help me learn how to write programs in Rust.

Arguments:
-l --long (false)   use long listing format
-a --all  (false)   do not ignore entries starting with \".\"
-s --sort (true)    sort names lexicographically
-h --help           display this help dialog
";
    println!("{}", message);
}

fn _parse(args: &mut Vec<String>) -> Result<CommandOptions, ParserError> {
    let default_path = String::from(".");
    let mut path: Option<String> = None;
    let mut show_long = false;
    let mut show_all = false;
    let mut show_help = false;
    let mut sort_lex = true;
    // command name
    args.remove(0);
    // process remaining flags
    while let Some(next) = args.pop() {
        match next.as_str() {
            "-l" => show_long = true,
            "--long" => show_long = true,
            "-a" => show_all = true,
            "--all" => show_all = true,
            "-s" => sort_lex = true,
            "--sort" => sort_lex = true,
            "-h" => show_help = true,
            "--help" => show_help = true,
            value => match path {
                Some(_) => {
                    return Err(ParserError {
                        message: String::from(value),
                    })
                }
                None => {
                    path = Some(value.to_string());
                }
            },
        }
    }
    Ok(CommandOptions {
        path: path.unwrap_or(default_path),
        show_long,
        show_all,
        show_help,
        sort_lex,
    })
}

pub fn parse(args: &mut Vec<String>) -> CommandOptions {
    match _parse(args) {
        Ok(opts) => {
            if opts.show_help {
                show_usage();
                exit(0)
            }
            opts
        }
        Err(e) => {
            println!("{}", e);
            show_usage();
            exit(1);
        }
    }
}
