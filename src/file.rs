use std::path::Path;

use crate::support::{get_created_time, get_file_name, get_modified_time, parse_permissions};

const SIZE_HELPER: [char; 6] = ['b', 'k', 'm', 'g', 't', 'p'];

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct File {
    pub name: String,
    size: Option<u64>,
    permissions: Option<String>,
    created_time: Option<String>,
    modified_time: Option<String>,
}

impl File {
    #[must_use]
    pub fn from(path: &Path, list: bool) -> Self {
        let metadata = path.metadata().ok();

        let name = get_file_name(path);

        let permissions = if list {
            Some(
                metadata
                    .clone()
                    .map_or("-".repeat(10), |meta| parse_permissions(&meta)),
            )
        } else {
            None
        };

        let size = if list {
            Some(metadata.map_or(0, |meta| meta.len()))
        } else {
            None
        };

        let created_time = if list {
            Some(get_created_time(path))
        } else {
            None
        };

        let modified_time = if list {
            Some(get_modified_time(path))
        } else {
            None
        };

        Self {
            name,
            size,
            permissions,
            created_time,
            modified_time,
        }
    }

    #[must_use]
    pub fn permissions(&self) -> String {
        self.permissions
            .as_deref()
            .map_or(String::new(), ToString::to_string)
    }

    fn size(&self) -> u64 {
        self.size.map_or(0, |num| num)
    }

    #[must_use]
    pub fn created_time(&self) -> String {
        self.created_time
            .as_deref()
            .map_or(String::new(), ToString::to_string)
    }

    #[must_use]
    pub fn formatted_size(&self) -> String {
        let size = self.size();
        let mut new_size = size;
        let mut post = 0;
        while new_size >= 1024 {
            new_size /= 1024;
            post += 1;
        }

        format!("{}{}", new_size, SIZE_HELPER[post])
    }

    #[must_use]
    pub fn modified_time(&self) -> String {
        self.modified_time
            .as_deref()
            .map_or(String::new(), ToString::to_string)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::file::File;

    fn create_temp_file() -> PathBuf {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("test_file.txt");
        std::fs::File::create(&file_path).expect("Failed to create temporary file");
        file_path
    }

    #[test]
    fn test_from() {
        let file_path = create_temp_file();

        let file = File::from(&file_path, true);

        assert_eq!(file.name, "test_file.txt");
        assert_eq!(file.size, Some(0));
        assert!(file.permissions.is_some());
        assert!(file.created_time.is_some());
        assert!(file.modified_time.is_some());
    }

    #[test]
    fn test_permissions() {
        let file = File {
            name: "test_file.txt".to_string(),
            size: Some(0),
            permissions: Some("rw-r--r--".to_string()),
            created_time: Some("2021-01-01".to_string()),
            modified_time: Some("2021-02-01".to_string()),
        };

        assert_eq!(file.permissions(), "rw-r--r--");
    }

    #[test]
    fn test_formatted_size() {
        let file = File {
            name: "test_file.txt".to_string(),
            size: Some(1024),
            permissions: None,
            created_time: None,
            modified_time: None,
        };

        assert_eq!(file.formatted_size(), "1k");
    }

    #[test]
    fn test_created_time() {
        let file = File {
            name: "test_file.txt".to_string(),
            size: Some(0),
            permissions: None,
            created_time: Some("2021-01-01".to_string()),
            modified_time: None,
        };

        assert_eq!(file.created_time(), "2021-01-01");
    }

    #[test]
    fn test_modified_time() {
        let file = File {
            name: "test_file.txt".to_string(),
            size: Some(0),
            permissions: None,
            created_time: None,
            modified_time: Some("2021-02-01".to_string()),
        };

        assert_eq!(file.modified_time(), "2021-02-01");
    }
}
