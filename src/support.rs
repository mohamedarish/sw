use std::{fs::Metadata, os::unix::prelude::PermissionsExt};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Folder {
    pub name: String,
    pub size: Option<u64>,
    pub permissions: Option<String>,
    pub children: Option<usize>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct File {
    pub name: String,
    pub size: Option<u64>,
    pub permissions: Option<String>,
}

impl Folder {
    pub fn from(
        name: String,
        size: Option<u64>,
        permissions: Option<String>,
        children: Option<usize>,
    ) -> Self {
        Self {
            name,
            size,
            permissions,
            children,
        }
    }

    pub fn permissions(&self) -> String {
        self.permissions
            .as_deref()
            .ok_or("No permissions initialized for Folder")
            .expect("Cannot dereference the field permissions")
            .to_string()
    }

    pub fn size(&self) -> u64 {
        self.size
            .ok_or("No size initialized")
            .expect("Cannot dereference the field size")
    }

    pub fn children(&self) -> usize {
        self.children
            .ok_or("No number of children initialized")
            .expect("Cannot dereference the field children")
    }
}

impl File {
    pub fn from(name: String, size: Option<u64>, permissions: Option<String>) -> Self {
        Self {
            name,
            size,
            permissions,
        }
    }

    pub fn permissions(&self) -> String {
        self.permissions
            .as_deref()
            .ok_or("No permissions initialized for Folder")
            .expect("Cannot dereference the field permissions")
            .to_string()
    }

    pub fn size(&self) -> u64 {
        self.size
            .ok_or("No size initialized")
            .expect("Cannot dereference the field size")
    }
}

pub fn parse_permissions(metadata: Metadata) -> String {
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

#[cfg(test)]
mod tests {
    use std::{fs, io, path::PathBuf};

    use tempfile::TempDir;

    use crate::{
        support::{parse_permissions, triplet},
        File, Folder,
    };

    fn create_test_directory(hidden: bool) -> io::Result<TempDir> {
        let temp_dir = TempDir::new()?;

        if hidden {
            fs::create_dir(temp_dir.path().join(".hidden_folder"))?;
            fs::File::create(temp_dir.path().join(".hidden_file.txt"))?;
        } else {
            fs::create_dir(temp_dir.path().join("folder1"))?;
            fs::File::create(temp_dir.path().join("file1.txt"))?;
        }

        Ok(temp_dir)
    }

    #[test]
    fn test_parse_permissions() {
        let temp_file = create_test_directory(false).unwrap();
        let root = PathBuf::from(temp_file.path());
        let metadata = root.metadata().expect("Failed to get metadata");

        let parsed_permissions = parse_permissions(metadata);

        assert_eq!(parsed_permissions, String::from("drwxr-xr-x"));
    }

    // Test the 'triplet' function
    #[test]
    fn test_triplet() {
        let mode = 256 | 128 | 64;
        let result = triplet(mode, 256, 128, 64);

        assert_eq!(&result, "rwx");
    }

    #[test]
    fn test_folder_from() {
        let folder = Folder::from(
            "my_folder".to_string(),
            Some(1024),
            Some("rwx".to_string()),
            None,
        );

        assert_eq!(folder.name, "my_folder");
        assert_eq!(folder.size, Some(1024));
        assert_eq!(folder.permissions, Some("rwx".to_string()));
    }

    // Test the 'File::from' function
    #[test]
    fn test_file_from() {
        let file = File::from(
            "my_file.txt".to_string(),
            Some(512),
            Some("rw-".to_string()),
        );

        assert_eq!(file.name, "my_file.txt");
        assert_eq!(file.size, Some(512));
        assert_eq!(file.permissions, Some("rw-".to_string()));
    }
}
