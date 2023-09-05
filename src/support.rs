use std::{fs::Metadata, os::unix::prelude::PermissionsExt, path::Path};

use chrono::{DateTime, Datelike, Timelike, Utc};

const DATE_MAPPER: [&str; 13] = [
    "-", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dev",
];

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
    let time = path
        .metadata()
        .map_or(DateTime::<Utc>::default(), |metadata| {
            metadata
                .created()
                .map_or(DateTime::<Utc>::default(), |time| {
                    DateTime::<Utc>::from(time)
                })
        });

    format!(
        "{: >3} {: >2} {: >2}:{: <2}",
        DATE_MAPPER
            .iter()
            .nth(time.month().try_into().map_or(0, |index| index))
            .map_or("-", |month| month),
        time.day(),
        time.hour(),
        time.minute()
    )
}

#[must_use]
pub fn get_modified_time(path: &Path) -> String {
    let time = path
        .metadata()
        .map_or(DateTime::<Utc>::default(), |metadata| {
            metadata
                .modified()
                .map_or(DateTime::<Utc>::default(), |time| {
                    DateTime::<Utc>::from(time)
                })
        });

    format!(
        "{: >3} {: >2} {: >2}:{: <2}",
        DATE_MAPPER
            .iter()
            .nth(time.month().try_into().map_or(0, |index| index))
            .map_or("-", |month| month),
        time.day(),
        time.hour(),
        time.minute()
    )
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    use crate::support::{get_created_time, get_file_name, get_modified_time};

    fn create_temp_file_with_permissions() -> PathBuf {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("test_file.txt");
        let mut file = File::create(&file_path).expect("Failed to create temporary file");

        file.write_all(b"Hello, World!")
            .expect("Failed to write to file");

        file_path
    }

    #[test]
    fn test_get_file_name() {
        let file_path = PathBuf::from("/path/to/some/file.txt");
        let file_name = get_file_name(&file_path);
        assert_eq!(file_name, "file.txt");
    }

    #[test]
    fn test_get_created_time() {
        let file_path = create_temp_file_with_permissions();

        let created_time = get_created_time(&file_path);

        assert!(!created_time.is_empty());
    }

    #[test]
    fn test_get_modified_time() {
        let file_path = create_temp_file_with_permissions();

        let modified_time = get_modified_time(&file_path);

        assert!(!modified_time.is_empty());
    }
}
