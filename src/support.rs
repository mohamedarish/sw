use std::{fs::Metadata, os::unix::prelude::PermissionsExt, path::Path};

use chrono::{DateTime, Utc};

const SIZE_HELPER: [char; 6] = ['b', 'k', 'm', 'g', 't', 'p'];

#[must_use]
pub fn parse_permissions(metadata: &Metadata) -> String {
    [
        if metadata.is_dir() {
            String::from("d")
        } else {
            String::from("-")
        },
        triplet(metadata.permissions().mode(), 256, 128, 64),
        triplet(metadata.permissions().mode(), 32, 16, 8),
        triplet(metadata.permissions().mode(), 4, 2, 1),
    ]
    .join("")
}

#[must_use]
pub fn convert_size(size: u64) -> String {
    let mut new_size = size;
    let mut post = 0;
    while new_size > 1024 {
        new_size /= 1024;
        post += 1;
    }

    format!("{}{}", new_size, SIZE_HELPER[post])
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

#[must_use]
pub fn get_file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || String::from("-"),
        |file_name| {
            file_name
                .to_str()
                .map_or_else(|| String::from("-"), ToString::to_string)
        },
    )
}

#[must_use]
pub fn get_created_time(path: &Path) -> String {
    path.metadata()
        .map_or(DateTime::<Utc>::default().to_rfc2822(), |metadata| {
            metadata
                .created()
                .map_or(DateTime::<Utc>::default().to_rfc2822(), |time| {
                    DateTime::<Utc>::from(time).to_rfc2822()
                })
        })
}

#[must_use]
pub fn get_modified_time(path: &Path) -> String {
    path.metadata()
        .map_or(DateTime::<Utc>::default().to_rfc2822(), |metadata| {
            metadata
                .modified()
                .map_or(DateTime::<Utc>::default().to_rfc2822(), |time| {
                    DateTime::<Utc>::from(time).to_rfc2822()
                })
        })
}
