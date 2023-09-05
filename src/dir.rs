use std::{
    collections::BTreeSet,
    io::{StdoutLock, Write},
    path::Path,
};

use crate::{file::File, folder::Folder, support::get_file_name, Error, Result};

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
    /// # Errors
    /// This would throw an error if it cannot resolve any required objects
    pub fn from(root: &Path, hidden: bool, list: bool) -> Result<Self> {
        let mut folders = BTreeSet::new();
        let mut hidden_folders = BTreeSet::new();
        let mut files = BTreeSet::new();
        let mut hidden_files = BTreeSet::new();

        let mut largest_name = if hidden { 2 } else { 0 };

        let (cur_dir, parent_dir) = if hidden {
            (
                Some(Folder::from(root, list)),
                Some(root.parent().map_or_else(
                    || Folder::from(root, list),
                    |parent| Folder::from(parent, list),
                )),
            )
        } else {
            (None, None)
        };

        let Some(directory_to_read) = root.read_dir().ok() else {
            return Err(Error::from("Cannot read the items in the directory"));
        };

        for item in directory_to_read {
            let Ok(item) = item else {
                return Err(Error::from("Cannot read the item in the directory"));
            };

            let path = item.path();

            let name = get_file_name(&path);

            if name.len() > largest_name {
                largest_name = name.len();
            }

            if !hidden && name.chars().nth(0) == Some('.') {
                continue;
            }

            if hidden && name.chars().nth(0) == Some('.') {
                if path.is_file() {
                    hidden_files.insert(File::from(&path, list));
                } else if path.is_dir() {
                    hidden_folders.insert(Folder::from(&path, list));
                }
            } else if path.is_file() {
                files.insert(File::from(&path, list));
            } else if path.is_dir() {
                folders.insert(Folder::from(&path, list));
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

    /// # Errors
    /// Will return an error if it cannot print to stdout 
    pub fn display_output(
        &self,
        stdout: &mut StdoutLock,
        width: usize,
        all: bool,
        list: bool,
    ) -> Result<()> {
        if list {
            self.print_list(stdout, all)?;
        } else {
            self.print_nlist(stdout, width, all)?;
        }

        Ok(())
    }

    fn print_nlist(&self, stdout: &mut StdoutLock, width: usize, all: bool) -> Result<()> {
        let mut count = 0;

        if !self.hidden_folders.is_empty() || !self.folders.is_empty() {
            if all {
                self.print_hidden_folders(stdout, &mut count, width)?;
            }

            self.print_visible_folders(stdout, &mut count, width)?;
        }

        if !self.hidden_files.is_empty() || !self.files.is_empty() {
            if all {
                self.print_hidden_files(stdout, &mut count, width)?;
            }

            self.print_visible_files(stdout, &mut count, width)?;

            match writeln!(stdout) {
                Ok(()) => {}
                Err(_) => return Err(Error::from("Cannot display output in stdout")),
            }
        }
        Ok(())
    }

    fn print_list(&self, stdout: &mut StdoutLock, all: bool) -> Result<()> {
        if !self.hidden_folders.is_empty() || !self.folders.is_empty() {
            if all {
                self.print_hidden_folders_list(stdout)?;
            }

            self.print_visible_folders_list(stdout)?;
        }

        if !self.hidden_files.is_empty() || !self.files.is_empty() {
            if all {
                self.print_hidden_files_list(stdout)?;
            }

            self.print_visible_files_list(stdout)?;
        }

        Ok(())
    }

    fn print_hidden_folders(
        &self,
        stdout: &mut StdoutLock,
        count: &mut usize,
        width: usize,
    ) -> Result<()> {
        for file in &self.hidden_folders {
            if width - *count < self.max_space() + 4 {
                match writeln!(stdout) {
                    Ok(()) => {}
                    Err(_) => return Err(Error::from("Cannot display output in stdout")),
                }
                *count = 0;
            }

            match write!(
                stdout,
                "\x1B[1;92m\u{ea83} \x1B[0 \x1B[1;96m{}{}\x1B[0",
                file.name,
                " ".repeat(self.max_space() - file.name.len())
            ) {
                Ok(()) => {}
                Err(_) => return Err(Error::from("Cannot display output in stdout")),
            }

            *count += self.max_space() + 4;
        }

        Ok(())
    }

    fn print_visible_folders(
        &self,
        stdout: &mut StdoutLock,
        count: &mut usize,
        width: usize,
    ) -> Result<()> {
        for file in &self.folders {
            if width - *count < self.max_space() + 4 {
                match writeln!(stdout) {
                    Ok(()) => {}
                    Err(_) => return Err(Error::from("Cannot display output in stdout")),
                }
                *count = 0;
            }

            match write!(
                stdout,
                "\x1B[92m\u{ea83} \x1B[0 \x1B[32m{}{}\x1B[0",
                file.name,
                " ".repeat(self.max_space() - file.name.len())
            ) {
                Ok(()) => {}
                Err(_) => return Err(Error::from("Cannot display output in stdout")),
            }

            *count += self.max_space() + 4;
        }

        Ok(())
    }

    fn print_hidden_files(
        &self,
        stdout: &mut StdoutLock,
        count: &mut usize,
        width: usize,
    ) -> Result<()> {
        for file in &self.hidden_files {
            if width - *count < self.max_space() + 4 {
                match writeln!(stdout) {
                    Ok(()) => {}
                    Err(_) => return Err(Error::from("Cannot display output in stdout")),
                }
                *count = 0;
            }

            match write!(
                stdout,
                "\x1B[94m\u{ea7b} \x1B[0 \x1B[96m{}{}\x1B[0",
                file.name,
                " ".repeat(self.max_space() - file.name.len())
            ) {
                Ok(()) => {}
                Err(_) => return Err(Error::from("Cannot display output in stdout")),
            }

            *count += self.max_space() + 4;
        }

        Ok(())
    }

    fn print_visible_files(
        &self,
        stdout: &mut StdoutLock,
        count: &mut usize,
        width: usize,
    ) -> Result<()> {
        for file in &self.files {
            if width - *count < self.max_space() + 4 {
                match writeln!(stdout) {
                    Ok(()) => {}
                    Err(_) => return Err(Error::from("Cannot display output in stdout")),
                }
                *count = 0;
            }

            match write!(
                stdout,
                "\x1B[94m\u{ea7b} \x1B[0 \x1B[0m{}{}\x1B[0",
                file.name,
                " ".repeat(self.max_space() - file.name.len())
            ) {
                Ok(()) => {}
                Err(_) => return Err(Error::from("Cannot display output in stdout")),
            }

            *count += self.max_space() + 4;
        }

        Ok(())
    }

    fn print_hidden_folders_list(&self, stdout: &mut StdoutLock) -> Result<()> {
        let Some(cur_dir) = self.cur_dir.as_ref() else {
            return Err(Error::from("Cannot reference current directory object"));
        };

        if cur_dir.name != "-" {
            let Some(parent_dir) = self.parent_dir.as_ref() else {
                return Err(Error::from("Cannot reference parent directory object"));
            };

            Self::print_list_folder(parent_dir, stdout)?;
        }

        Self::print_list_folder(cur_dir, stdout)?;

        for file in &self.hidden_folders {
            Self::print_list_folder(file, stdout)?;
        }

        Ok(())
    }

    fn print_visible_folders_list(&self, stdout: &mut StdoutLock) -> Result<()> {
        for file in &self.folders {
            match Self::print_list_folder(file, stdout) {
                Ok(()) => {}
                Err(_) => return Err(Error::from("Cannot write to stdout")),
            }
        }

        Ok(())
    }

    fn print_hidden_files_list(&self, stdout: &mut StdoutLock) -> Result<()> {
        for file in &self.hidden_files {
            match Self::print_list_file(file, stdout) {
                Ok(()) => {}
                Err(_) => return Err(Error::from("Cannot write to stdout")),
            }
        }

        Ok(())
    }

    fn print_visible_files_list(&self, stdout: &mut StdoutLock) -> Result<()> {
        for file in &self.files {
            match Self::print_list_file(file, stdout) {
                Ok(()) => {}
                Err(_) => return Err(Error::from("Cannot write ro stdout")),
            }
        }

        Ok(())
    }

    fn print_list_file(file: &File, stdout: &mut StdoutLock) -> Result<()> {
        match writeln!(
            stdout,
            "\x1B[0m{: <10} {: <4}{: <6} {} \x1B[0 \x1B[94m\u{ea7b} \x1B[0 \x1B[34m{: <25} \x1B[0",
            file.permissions(),
            1,
            file.formatted_size(),
            file.created_time(),
            file.name
        ) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::from("Cannot write to stdout")),
        }
    }

    fn print_list_folder(file: &Folder, stdout: &mut StdoutLock) -> Result<()> {
        match writeln!(
            stdout,
            "\x1B[0m{: <10} {: <4}{: <6} {} \x1B[0 \x1B[92m\u{ea83} \x1B[0 \x1B[1;32m{: <25} \x1B[0",
            file.permissions(),
            file.children(),
            '-',
            file.created_time(), 
            file.name
        ) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::from("Cannot write to stdout")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, path::PathBuf};

    use crate::dir::Directory;

    fn create_temp_directory_structure() -> PathBuf {
        let root_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let dir1 = root_dir.path().join("dir1");
        std::fs::create_dir(&dir1).expect("Failed to create directory");
        let dir2 = root_dir.path().join("dir2");
        std::fs::create_dir(dir2).expect("Failed to create directory");
        let file1 = dir1.join("file1.txt");
        std::fs::File::create(file1).expect("Failed to create file");
        let file2 = dir1.join("file2.txt");
        std::fs::File::create(file2).expect("Failed to create file");

        root_dir.into_path()
    }

    #[test]
    fn test_from() {
        let root_dir = create_temp_directory_structure();

        let directory =
            Directory::from(&root_dir, false, true).expect("Failed to create Directory");

        assert!(directory.cur_dir.is_none());
        assert!(directory.parent_dir.is_none());
        assert_eq!(directory.folders.len(), 2);
        assert_eq!(directory.hidden_folders.len(), 0);
        assert_eq!(directory.files.len(), 0);
        assert_eq!(directory.hidden_files.len(), 0);
        assert_eq!(directory.largest_name, 4);

        std::fs::remove_dir_all(&root_dir).expect("Failed to remove temporary directory");
    }

    #[test]
    fn test_max_space() {
        let directory = Directory {
            cur_dir: None,
            parent_dir: None,
            folders: BTreeSet::new(),
            hidden_folders: BTreeSet::new(),
            files: BTreeSet::new(),
            hidden_files: BTreeSet::new(),
            largest_name: 10,
        };

        assert_eq!(directory.max_space(), 14);
    }
}
