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

// https://www.gnu.org/software/libc/manual/html_node/Permission-Bits.html
const S_IRUSR: u32 = 0o0400;
const S_IWUSR: u32 = 0o0200;
const S_IXUSR: u32 = 0o0100;
const S_ISUID: u32 = 0o04000;
// const S_RWXU: u32 = (S_IRUSR | S_IWUSR | S_IXUSR);

const S_IRGRP: u32 = 0o040;
const S_IWGRP: u32 = 0o020;
const S_IXGRP: u32 = 0o010;
const S_ISGID: u32 = 0o02000;
// const S_RWXG: u32 = (S_IRGRP | S_IWGRP | S_IXGRP);

const S_IROTH: u32 = 0o04;
const S_IWOTH: u32 = 0o02;
const S_IXOTH: u32 = 0o01;
// const S_RWXO: u32 = (S_IROTH | S_IWOTH | S_IXOTH);

const S_ISVTX: u32 = 0o01000;

pub struct File {
    pub name: String,
    pub kind: Kind,
    pub size: u64,
    pub ino: u64,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub atime: i64,
    pub modified: SystemTime,
}

// to_unix_permission converts the octal mode
// into the unix style rwx... permission style
pub fn to_unix_permission(mode: u32) -> String {
    let mut result = String::with_capacity(9);
    // USER
    if (mode & S_IRUSR) != 0 {
        result.push('r')
    } else {
        result.push('-')
    }
    if (mode & S_IWUSR) != 0 {
        result.push('w')
    } else {
        result.push('-')
    }
    if (mode & S_ISUID) != 0 {
        if (mode & S_IXUSR) != 0 {
            result.push('s')
        } else {
            result.push('S')
        }
    } else if (mode & S_IXUSR) != 0 {
        result.push('x')
    } else {
        result.push('-')
    }
    // GROUP
    if (mode & S_IRGRP) != 0 {
        result.push('r')
    } else {
        result.push('-')
    }
    if (mode & S_IWGRP) != 0 {
        result.push('w')
    } else {
        result.push('-')
    }
    if (mode & S_ISGID) != 0 {
        if (mode & S_IXGRP) != 0 {
            result.push('s')
        } else {
            result.push('S')
        }
    } else if (mode & S_IXGRP) != 0 {
        result.push('x')
    } else {
        result.push('-')
    }
    // OTHER
    if (mode & S_IROTH) != 0 {
        result.push('r')
    } else {
        result.push('-')
    }
    if (mode & S_IWOTH) != 0 {
        result.push('w')
    } else {
        result.push('-')
    }
    if (mode & S_ISVTX) != 0 {
        if (mode & S_IXOTH) != 0 {
            result.push('t')
        } else {
            result.push('T')
        }
    } else if (mode & S_IXOTH) != 0 {
        result.push('x')
    } else {
        result.push('-')
    }
    result
}

pub fn resolve_user(id: u32) -> String {
    resolve_id(id, "/etc/passwd")
}
pub fn resolve_group(id: u32) -> String {
    resolve_id(id, "/etc/group")
}

// resolve_id will return a matching user or group
// identifier if it exists on the host filesystem.
fn resolve_id(id: u32, file_path: &str) -> String {
    match std::fs::read_to_string(file_path) {
        Ok(value) => {
            let lines: Vec<&str> = value.split('\n').collect();
            let username: String = lines
                .iter()
                .filter_map(|line| {
                    let line_split: Vec<&str> = line.split(':').collect();
                    if line_split.len() > 2 {
                        let user_or_group = line_split.get(0).unwrap().to_string();
                        let other_id_str = line_split.get(2).unwrap().to_string();
                        match other_id_str.parse::<u32>() {
                            Ok(user_id) => {
                                if user_id == id {
                                    Some(user_or_group)
                                } else {
                                    None
                                }
                            }
                            Err(_) => None,
                        }
                    } else {
                        None
                    }
                })
                .collect();
            username
        }
        Err(_) => String::from("????"),
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
    let modified = md.modified().unwrap();
    File {
        name: file_name,
        kind: ftype,
        size: md.len(),
        ino: md.st_ino(),
        uid: md.st_uid(),
        gid: md.st_gid(),
        mode: perms.mode(),
        atime: md.st_atime(),
        modified,
    }
}

pub fn get_files(path: String, show_all: bool, sort_lex: bool) -> std::io::Result<Vec<File>> {
    let entries = read_dir(path)?;
    let mut files: Vec<File> = entries
        .map(|entry| to_file(entry.unwrap()))
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
