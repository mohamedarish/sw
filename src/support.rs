use std::{fs::Metadata, os::unix::prelude::PermissionsExt};

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

pub struct Folder {
    name: String,
    size: Option<u32>,
}

pub struct File {
    name: String,
    size: Option<u32>,
}

impl Folder {
    pub fn from(name: String, size: Option<u32>) -> Self {
        Self { name, size }
    }
}

impl File {
    pub fn from(name: String, size: Option<u32>) -> Self {
        Self { name, size }
    }
}

pub fn parse_permissions(metadata: Metadata) -> String {
    [
        if metadata.is_dir() {
            String::from("d")
        } else {
            String::from("-")
        },
        triplet(
            metadata.permissions().mode() as u16,
            S_IRUSR,
            S_IWUSR,
            S_IXUSR,
        ),
        triplet(
            metadata.permissions().mode() as u16,
            S_IRGRP,
            S_IWGRP,
            S_IXGRP,
        ),
        triplet(
            metadata.permissions().mode() as u16,
            S_IROTH,
            S_IWOTH,
            S_IXOTH,
        ),
    ]
    .join("")
}

fn triplet(mode: u16, read: u16, write: u16, execute: u16) -> String {
    String::from(match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (0, 0, _) => "--x",
        (0, _, 0) => "-w-",
        (_, 0, 0) => "r--",
        (0, _, _) => "-wx",
        (_, 0, _) => "r-x",
        (_, _, 0) => "rw-",
        (_, _, _) => "rwx",
    })
}
