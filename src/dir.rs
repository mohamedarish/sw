use colored::Colorize;
use std::{
    collections::BTreeSet,
    fs::ReadDir,
    io::{StdoutLock, Write},
};

use crate::support::parse_permissions;

pub struct Directory {
    pub folders: BTreeSet<String>,
    pub hidden_folders: BTreeSet<String>,
    pub files: BTreeSet<String>,
    pub hidden_files: BTreeSet<String>,
}

impl Directory {
    pub fn from(root: ReadDir, hidden: bool, list: bool) -> Self {
        let mut folders = BTreeSet::new();
        let mut hidden_folders = BTreeSet::new();
        let mut files = BTreeSet::new();
        let mut hidden_files = BTreeSet::new();

        for item in root {
            let item = item.expect("cannot access file");
            let name = item
                .file_name()
                .into_string()
                .expect("Cannot convert to string");

            if !hidden && name.chars().nth(0) == Some('.') {
                continue;
            }

            let data = item.metadata().expect("Cannot access metadata");

            let perm_string = parse_permissions(data);

            println!("{perm_string}");

            let info = item.file_type().expect("Cannot access info of item");

            if hidden && name.chars().nth(0) == Some('.') {
                if info.is_file() {
                    hidden_files.insert(name.to_string());
                } else if info.is_dir() {
                    hidden_folders.insert(name.to_string());
                }
            } else if info.is_file() {
                files.insert(name.to_string());
            } else if info.is_dir() {
                folders.insert(name.to_string());
            }
        }

        Self {
            folders,
            hidden_folders,
            files,
            hidden_files,
        }
    }

    pub fn print_nlist(&self, stdout: &mut StdoutLock, width: usize, all: bool) {
        if !self.hidden_folders.is_empty() || !self.folders.is_empty() {
            let mut count = 0;
            if all {
                write!(stdout, "{: <25}", "..".bright_cyan().bold())
                    .expect("Cannot write to stdout");
                write!(stdout, "{: <25}", ".".bright_cyan().bold())
                    .expect("Cannot write to stdout");
                count += 50;

                for file in &self.hidden_folders {
                    if width - count < 40 {
                        writeln!(stdout).expect("Cannot write to stdout");
                        count = 0;
                    }

                    write!(stdout, "{: <25}", file.bright_cyan().bold())
                        .expect("Cannot write to stdout");

                    count += 25;
                }
            }

            for file in &self.folders {
                if width - count < 40 {
                    writeln!(stdout).expect("Cannot write to stdout");
                    count = 0;
                }

                write!(stdout, "{: <25}", file.green().bold()).expect("Cannot write to stdout");

                count += 25;
            }

            writeln!(stdout).expect("Cannot write to stdout");
        }

        if !self.hidden_files.is_empty() || !self.files.is_empty() {
            let mut count = 0;
            if all {
                for file in &self.hidden_files {
                    if width - count < 40 {
                        writeln!(stdout).expect("Cannot write to stdout");
                        count = 0;
                    }

                    write!(stdout, "{: <25}", file.bright_cyan()).expect("Cannot write to stdout");

                    count += 25;

                    if width - count < 40 {
                        writeln!(stdout).expect("Cannot write to stdout");
                        count = 0;
                    }
                }
            }

            for file in &self.files {
                if width - count < 40 {
                    writeln!(stdout).expect("Cannot write to stdout");
                    count = 0;
                }

                write!(stdout, "{: <25}", file).expect("Cannot write to stdout");

                count += 25;
            }

            writeln!(stdout).expect("Cannot write to stdout");
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeSet;
    use std::fs;
    use std::io;
    use tempfile::TempDir;

    use crate::Directory;

    fn create_test_directory(hidden: bool) -> io::Result<TempDir> {
        let temp_dir = TempDir::new()?;

        // Create some test files and directories

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
    fn test_directory_creation_visible() {
        let temp_dir = create_test_directory(false).unwrap();
        let root = temp_dir.path().read_dir().unwrap();

        let directory = Directory::from(root, false, false);

        let expected_folders = vec![String::from("folder1")];
        let expected_files = vec![String::from("file1.txt")];

        assert_eq!(directory.folders, BTreeSet::from_iter(expected_folders));
        assert_eq!(directory.hidden_folders, BTreeSet::new());
        assert_eq!(directory.files, BTreeSet::from_iter(expected_files));
        assert_eq!(directory.hidden_files, BTreeSet::new());
    }

    #[test]
    fn test_directory_creation_hidden() {
        let temp_dir = create_test_directory(true).unwrap();
        let root = temp_dir.path().read_dir().unwrap();

        let directory = Directory::from(root, true, false);

        let expected_hidden_folders = vec![String::from(".hidden_folder")];
        let expected_hidden_files = vec![String::from(".hidden_file.txt")];

        assert_eq!(directory.folders, BTreeSet::new());
        assert_eq!(
            directory.hidden_folders,
            BTreeSet::from_iter(expected_hidden_folders)
        );
        assert_eq!(directory.files, BTreeSet::new());
        assert_eq!(
            directory.hidden_files,
            BTreeSet::from_iter(expected_hidden_files)
        );
    }
}
