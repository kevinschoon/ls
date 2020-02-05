use std::fs::{read_dir, DirEntry};
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;
use std::time::SystemTime;

pub enum Kind {
    File,
    Dir,
    Link,
    Fifo,
    Block,
    Char,
    Socket,
}

pub struct File {
    pub name: String,
    pub kind: Kind,
    pub size: u64,
    pub ino: u64,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub modified: SystemTime,
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
        ino: md.st_ino(),
        uid: md.st_uid(),
        gid: md.st_gid(),
        mode: perms.mode(),
        modified: SystemTime::now(),
    }
}

pub fn get_files(path: String, show_all: bool, sort_lex: bool) -> std::io::Result<Vec<File>> {
    let entries = read_dir(path)?;
    let mut files: Vec<File> = entries
        .map(|entry| match entry {
            Ok(entry) => to_file(entry),
            e => panic!(e),
        })
        .filter(|file| {
            if !show_all && file.name.starts_with('.') {
                return false;
            }
            true
        })
        .collect();
    if sort_lex {
        files.sort_by(|a, b| a.name.cmp(&b.name));
    }
    Ok(files)
}
