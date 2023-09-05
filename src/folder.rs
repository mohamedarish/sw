use std::path::Path;

use crate::support::{get_created_time, get_file_name, parse_permissions};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Folder {
    pub name: String,
    permissions: Option<String>,
    children: Option<usize>,
    created_time: Option<String>,
}

impl Folder {
    #[must_use]
    pub fn from(path: &Path, list: bool) -> Self {
        let metadata = path.metadata().ok();

        let name = get_file_name(path);

        let permissions = if list {
            Some(metadata.map_or("-".repeat(10), |meta| parse_permissions(&meta)))
        } else {
            None
        };

        let children = if list {
            Some(path.read_dir().map_or(0, Iterator::count))
        } else {
            None
        };

        let created_time = if list {
            Some(get_created_time(path))
        } else {
            None
        };

        Self {
            name,
            permissions,
            children,
            created_time,
        }
    }

    #[must_use]
    pub fn permissions(&self) -> String {
        self.permissions
            .as_deref()
            .map_or("-".repeat(10), ToString::to_string)
    }

    #[must_use]
    pub fn children(&self) -> usize {
        self.children.map_or(0, |num| num)
    }

    #[must_use]
    pub fn created_time(&self) -> String {
        self.created_time
            .as_deref()
            .map_or(String::new(), ToString::to_string)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, DirBuilder};
    use std::io::Write;
    use std::path::PathBuf;

    use crate::folder::Folder;

    fn create_temp_directory_structure() -> PathBuf {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

        let dir1 = temp_dir.path().join("dir1");
        DirBuilder::new()
            .create(dir1)
            .expect("Failed to create directory");
        let dir2 = temp_dir.path().join("dir2");
        DirBuilder::new()
            .create(dir2)
            .expect("Failed to create directory");
        let file1 = temp_dir.path().join("file1.txt");
        let mut file = fs::File::create(file1).expect("Failed to create file");
        file.write_all(b"Hello, World!")
            .expect("Failed to write to file");

        temp_dir.into_path()
    }

    #[test]
    fn test_from() {
        let dir_path = create_temp_directory_structure();

        let folder = Folder::from(&dir_path, true);

        assert!(folder.permissions.is_some());
        assert_eq!(folder.children, Some(3));
        assert!(folder.created_time.is_some());
    }

    #[test]
    fn test_permissions() {
        let folder = Folder {
            name: "test_folder".to_string(),
            permissions: Some("rw-r--r--".to_string()),
            children: None,
            created_time: None,
        };

        assert_eq!(folder.permissions(), "rw-r--r--");
    }

    #[test]
    fn test_children() {
        let folder = Folder {
            name: "test_folder".to_string(),
            permissions: None,
            children: Some(5),
            created_time: None,
        };

        assert_eq!(folder.children(), 5);
    }

    #[test]
    fn test_created_time() {
        let folder = Folder {
            name: "test_folder".to_string(),
            permissions: None,
            children: None,
            created_time: Some("2021-01-01".to_string()),
        };

        assert_eq!(folder.created_time(), "2021-01-01");
    }
}
