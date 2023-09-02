use std::path::Path;

use crate::support::{get_created_time, get_file_name, get_modified_time, parse_permissions};

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

    #[must_use]
    pub fn size(&self) -> u64 {
        self.size.map_or(0, |num| num)
    }

    #[must_use]
    pub fn created_time(&self) -> String {
        self.created_time
            .as_deref()
            .map_or(String::new(), ToString::to_string)
    }

    #[must_use]
    pub fn modified_time(&self) -> String {
        self.modified_time
            .as_deref()
            .map_or(String::new(), ToString::to_string)
    }
}
