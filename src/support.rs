use std::{fs::Metadata, os::unix::prelude::PermissionsExt, path::Path};

use chrono::{DateTime, Utc};

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

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    use crate::support::{get_created_time, get_file_name, get_modified_time};

    fn create_temp_file_with_permissions(permissions: u32) -> PathBuf {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("test_file.txt");
        let mut file = File::create(&file_path).expect("Failed to create temporary file");

        file.write_all(b"Hello, World!")
            .expect("Failed to write to file");

        let metadata = fs::metadata(&file_path).expect("Failed to get file metadata");
        let mut permission = metadata.permissions();

        permission.set_mode(permissions);
        fs::set_permissions(&file_path, permission).expect("Failed to set file permissions");

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
        let file_path = create_temp_file_with_permissions(0o755);

        let created_time = get_created_time(&file_path);

        assert!(!created_time.is_empty());
    }

    #[test]
    fn test_get_modified_time() {
        let file_path = create_temp_file_with_permissions(0o755);

        let modified_time = get_modified_time(&file_path);

        assert!(!modified_time.is_empty());
    }
}
