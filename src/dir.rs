use std::{
    collections::BTreeSet,
    io::{StdoutLock, Write},
    path::Path,
};

use crate::{
    support::{convert_size, get_file_name, parse_permissions, File, Folder},
    Error, Result,
};

pub struct Directory {
    pub cur_dir: Option<Folder>,
    pub parent_dir: Option<Folder>,
    pub folders: BTreeSet<Folder>,
    pub hidden_folders: BTreeSet<Folder>,
    pub files: BTreeSet<File>,
    pub hidden_files: BTreeSet<File>,
    largest_name: usize,
}

impl Directory {
    /// # Panics
    /// This function panics if relevant information cannot be accessed
    ///
    /// # Errors
    /// Many errors
    pub fn from(root: &Path, hidden: bool, list: bool) -> Result<Self> {
        let mut folders = BTreeSet::new();
        let mut hidden_folders = BTreeSet::new();
        let mut files = BTreeSet::new();
        let mut hidden_files = BTreeSet::new();

        let mut largest_name = if hidden { 2 } else { 0 };

        let (cur_dir, parent_dir) = if hidden {
            let name = format!(".({})", get_file_name(root));

            let parent = root.parent();

            let parent_name = parent.map_or_else(
                || String::from(".."),
                |p| format!("..({})", get_file_name(p)),
            );

            let (permissions, children, parent_permissions, parent_children) = if list {
                let metadata = root.metadata().expect("Cannot access metadata");
                let parent_metadata = parent.map_or_else(|| None, |p| Some(p.metadata()));

                (
                    Some(parse_permissions(&metadata)),
                    Some(root.read_dir().expect("Cannot read directory").count()),
                    Some(parent_metadata.map_or(String::new(), |m| {
                        m.map_or(String::new(), |meta| parse_permissions(&meta))
                    })),
                    Some(parent.map_or(0, |p| p.read_dir().map_or(0, Iterator::count))),
                )
            } else {
                (None, None, None, None)
            };

            (
                Some(Folder::from(name, permissions, children)),
                Some(Folder::from(
                    parent_name,
                    parent_permissions,
                    parent_children,
                )),
            )
        } else {
            (None, None)
        };

        for item in root.read_dir().expect("Cannot read directory") {
            let item = item.expect("cannot access file");
            let name = get_file_name(&item.path());

            if name.len() > largest_name {
                largest_name = name.len();
            }

            if !hidden && name.chars().nth(0) == Some('.') {
                continue;
            }

            let (permissions, size, children) = if list {
                let metadata = item.metadata().expect("Cannot access metadata");

                let children = if metadata.is_dir() {
                    let directory = item.path().read_dir().map_or_else(|_| None, Some);

                    directory.map(Iterator::count)
                } else {
                    None
                };

                let size = if metadata.is_file() {
                    Some(metadata.len())
                } else {
                    None
                };

                (Some(parse_permissions(&metadata)), size, children)
            } else {
                (None, None, None)
            };

            let info = item.file_type().expect("Cannot access info of item");

            if hidden && name.chars().nth(0) == Some('.') {
                if info.is_file() {
                    hidden_files.insert(File::from(name.to_string(), size, permissions));
                } else if info.is_dir() {
                    hidden_folders.insert(Folder::from(name.to_string(), permissions, children));
                }
            } else if info.is_file() {
                files.insert(File::from(name.to_string(), size, permissions));
            } else if info.is_dir() {
                folders.insert(Folder::from(name.to_string(), permissions, children));
            }
        }

        Ok(Self {
            cur_dir,
            parent_dir,
            folders,
            hidden_folders,
            files,
            hidden_files,
            largest_name,
        })
    }

    const fn max_space(&self) -> usize {
        self.largest_name + 4
    }

    pub fn display_output(&self, stdout: &mut StdoutLock, width: usize, all: bool, list: bool) {
        if list {
            self.print_list(stdout, all);
        } else {
            self.print_nlist(stdout, width, all);
        }
    }

    fn print_nlist(&self, stdout: &mut StdoutLock, width: usize, all: bool) {
        let mut count = 0;

        if !self.hidden_folders.is_empty() || !self.folders.is_empty() {
            if all {
                self.print_hidden_folders(stdout, &mut count, width);
            }

            self.print_visible_folders(stdout, &mut count, width);
        }

        if !self.hidden_files.is_empty() || !self.files.is_empty() {
            if all {
                self.print_hidden_files(stdout, &mut count, width);
            }

            self.print_visible_files(stdout, &mut count, width);

            writeln!(stdout).expect("Cannot write to stdout");
        }
    }

    fn print_list(&self, stdout: &mut StdoutLock, all: bool) {
        if !self.hidden_folders.is_empty() || !self.folders.is_empty() {
            if all {
                self.print_hidden_folders_list(stdout);
            }

            self.print_visible_folders_list(stdout);
        }

        if !self.hidden_files.is_empty() || !self.files.is_empty() {
            if all {
                self.print_hidden_files_list(stdout);
            }

            self.print_visible_files_list(stdout);
        }
    }

    fn print_hidden_folders(&self, stdout: &mut StdoutLock, count: &mut usize, width: usize) {
        write!(
            stdout,
            "\x1B[1;92m\u{ea83} \x1B[0 \x1B[1;96m..{}\x1B[0",
            " ".repeat(self.max_space() - 2)
        )
        .expect("Cannot write to stdout");
        write!(
            stdout,
            "\x1B[1;92m\u{ea83} \x1B[0 \x1B[1;96m.{}\x1B[0",
            " ".repeat(self.max_space() - 1)
        )
        .expect("Cannot write to stdout");
        *count += self.max_space() * 2;

        for file in &self.hidden_folders {
            if width - *count < self.max_space() + 4 {
                writeln!(stdout).expect("Cannot write to stdout");
                *count = 0;
            }

            write!(
                stdout,
                "\x1B[1;92m\u{ea83} \x1B[0 \x1B[1;96m{}{}\x1B[0",
                file.name,
                " ".repeat(self.max_space() - file.name.len())
            )
            .expect("Cannot write to stdout");

            *count += self.max_space() + 4;
        }
    }

    fn print_visible_folders(&self, stdout: &mut StdoutLock, count: &mut usize, width: usize) {
        for file in &self.folders {
            if width - *count < self.max_space() + 4 {
                writeln!(stdout).expect("Cannot write to stdout");
                *count = 0;
            }

            write!(
                stdout,
                "\x1B[92m\u{ea83} \x1B[0 \x1B[32m{}{}\x1B[0",
                file.name,
                " ".repeat(self.max_space() - file.name.len())
            )
            .expect("Cannot write to stdout");

            *count += self.max_space() + 4;
        }
    }

    fn print_hidden_files(&self, stdout: &mut StdoutLock, count: &mut usize, width: usize) {
        for file in &self.hidden_files {
            if width - *count < self.max_space() + 4 {
                writeln!(stdout).expect("Cannot write to stdout");
                *count = 0;
            }

            write!(
                stdout,
                "\x1B[94m\u{ea7b} \x1B[0 \x1B[96m{}{}\x1B[0",
                file.name,
                " ".repeat(self.max_space() - file.name.len())
            )
            .expect("Cannot write to stdout");

            *count += self.max_space() + 4;
        }
    }

    fn print_visible_files(&self, stdout: &mut StdoutLock, count: &mut usize, width: usize) {
        for file in &self.files {
            if width - *count < self.max_space() + 4 {
                writeln!(stdout).expect("Cannot write to stdout");
                *count = 0;
            }

            write!(
                stdout,
                "\x1B[94m\u{ea7b} \x1B[0 \x1B[0m{}{}\x1B[0",
                file.name,
                " ".repeat(self.max_space() - file.name.len())
            )
            .expect("Cannot write to stdout");

            *count += self.max_space() + 4;
        }
    }

    fn print_hidden_folders_list(&self, stdout: &mut StdoutLock) {
        let parent_dir = self
            .parent_dir
            .as_ref()
            .ok_or("Cannot dereference the parent dir")
            .expect("The parent dir could not be successfully dereferenced");

        Self::print_list_folder(parent_dir, stdout).expect("Cannot write to stdout");

        let cur_dir = self
            .cur_dir
            .as_ref()
            .ok_or("Cannot dereference the current dir")
            .expect("The current dir could not be dereferences");

        Self::print_list_folder(cur_dir, stdout).expect("Cannot write to stdout");

        for file in &self.hidden_folders {
            Self::print_list_folder(file, stdout).expect("Cannot write to stdout");
        }
    }

    fn print_visible_folders_list(&self, stdout: &mut StdoutLock) {
        for file in &self.folders {
            Self::print_list_folder(file, stdout).expect("Cannot write to stdout");
        }
    }

    fn print_hidden_files_list(&self, stdout: &mut StdoutLock) {
        for file in &self.hidden_files {
            Self::print_list_file(file, stdout).expect("Cannot write to stdout");
        }
    }

    fn print_visible_files_list(&self, stdout: &mut StdoutLock) {
        for file in &self.files {
            Self::print_list_file(file, stdout).expect("Cannot write to stdout");
        }
    }

    fn print_list_file(file: &File, stdout: &mut StdoutLock) -> Result<()> {
        match writeln!(
            stdout,
            "\x1B[0m{} {: <4}{: <6}\x1B[0 \x1B[94m\u{ea7b} \x1B[0 \x1B[34m{: <25} \x1B[0",
            file.permissions(),
            1,
            convert_size(file.size()),
            file.name
        ) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::from("Cannot write to stdout")),
        }
    }

    fn print_list_folder(file: &Folder, stdout: &mut StdoutLock) -> Result<()> {
        match writeln!(
            stdout,
            "\x1B[0m{} {: <4}{: <6}\x1B[0 \x1B[92m\u{ea83} \x1B[0 \x1B[1;32m{: <25} \x1B[0",
            file.permissions(),
            file.children(),
            '-',
            file.name
        ) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::from("Cannot write to stdout")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use tempfile::TempDir;

    use crate::dir::Directory;
    use crate::support::File;
    use crate::support::Folder;

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
    fn test_directory_creation_visible() {
        let temp_dir = create_test_directory(false).expect("Cannot create a test directory");
        let root = PathBuf::from(temp_dir.path());

        let directory = Directory::from(&root, false, false).expect("Some error occurred");

        let expected_folders = vec![Folder::from("folder1".to_string(), None, None)];
        let expected_files = vec![File::from("file1.txt".to_string(), None, None)];

        assert_eq!(directory.folders, BTreeSet::from_iter(expected_folders));
        assert_eq!(directory.hidden_folders, BTreeSet::new());
        assert_eq!(directory.files, BTreeSet::from_iter(expected_files));
        assert_eq!(directory.hidden_files, BTreeSet::new());
    }

    #[test]
    fn test_directory_creation_hidden() {
        let temp_dir = create_test_directory(true).expect("Cannot create a test directory");
        let root = PathBuf::from(temp_dir.path());

        let directory = Directory::from(&root, true, false).expect("Some error occurred");

        let expected_hidden_folders = vec![Folder::from(".hidden_folder".to_string(), None, None)];
        let expected_hidden_files = vec![File::from(".hidden_file.txt".to_string(), None, None)];

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
