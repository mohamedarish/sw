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
            metadata.permissions().mode(),
            S_IRUSR as u32,
            S_IWUSR as u32,
            S_IXUSR as u32,
        ),
        triplet(
            metadata.permissions().mode(),
            S_IRGRP as u32,
            S_IWGRP as u32,
            S_IXGRP as u32,
        ),
        triplet(
            metadata.permissions().mode(),
            S_IROTH as u32,
            S_IWOTH as u32,
            S_IXOTH as u32,
        ),
    ]
    .join("")
}

fn triplet(mode: u32, read: u32, write: u32, execute: u32) -> String {
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
