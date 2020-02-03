use std::fs::{read_dir, DirEntry};
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;
use std::time::SystemTime;

#[derive(Debug)]
pub enum Kind {
    File,
    Dir,
    Link,
    Fifo,
    Block,
    Char,
    Socket,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub kind: Kind,
    pub size: u64,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub modified: SystemTime,
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

pub fn get_files(path: String, show_all: bool) -> std::io::Result<Vec<File>> {
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
